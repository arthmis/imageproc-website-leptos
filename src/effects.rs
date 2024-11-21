use std::rc::Rc;
use std::str::FromStr;

use js_sys::{ArrayBuffer, Reflect, Uint8ClampedArray};
use leptos::{
    create_signal,
    html::{Canvas, Img},
    Effect, NodeRef, ReadSignal, SignalSet,
};
use log::info;
use shared::WorkerResponseMessage;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{
    window, CanvasRenderingContext2d, Event, ImageData, MediaQueryListEvent, MessageEvent, Worker,
    WorkerOptions, WorkerType,
};

use crate::get_scaled_image_dimensions_to_canvas;

pub fn use_resize(image_ref: NodeRef<Img>, selected_image_canvas: NodeRef<Canvas>) {
    let resize_closure: Closure<dyn FnMut(Event)> = Closure::new(move |_event: Event| {
        log::debug!("resizing");
        let image_node = image_ref.get_untracked().unwrap();
        let canvas = selected_image_canvas.get_untracked().unwrap();

        let (scaled_width, scaled_height) =
            get_scaled_image_dimensions_to_canvas(&image_node, &canvas);

        log::debug!("width: {}, height: {}", scaled_width, scaled_height);
        // TODO: next step
        // think about sending a message to the worker on every resize event(debounced) and the worker
        // will send back a copy of the original image that has potentially been modified and then resize that
        // if I don't do that then I can try resizing the image that is already in the canvas which might be faster and hopefully better
        let new_canvas_width = canvas.client_width();
        let new_canvas_height = canvas.client_height();

        canvas.set_width(new_canvas_width as u32);
        // TODO: setting the height directly with using .offset_height() or any other height
        // functions
        // doesn't work correctly. However if I place the value into a variable first then it works
        // no idea how this is happening
        canvas.set_height(new_canvas_height as u32);

        let center_x = (canvas.width() as f64 - scaled_width) / 2.;
        let center_y = (canvas.height() as f64 - scaled_height) / 2.;
        let canvas_context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        canvas_context
            .draw_image_with_html_image_element_and_dw_and_dh(
                &image_node,
                center_x,
                center_y,
                scaled_width,
                scaled_height,
            )
            .unwrap();
    });

    window()
        .unwrap()
        .add_event_listener_with_callback("resize", resize_closure.as_ref().unchecked_ref())
        .unwrap();

    resize_closure.forget();
}
pub fn use_worker(selected_image_canvas: NodeRef<Canvas>) -> Rc<Worker> {
    let on_worker_message: Closure<dyn FnMut(MessageEvent)> =
        Closure::new(move |message_event: MessageEvent| {
            let message = &Reflect::get(&message_event.data(), &JsValue::from_str("message"))
                .unwrap()
                .as_string()
                .unwrap();
            let worker_message = WorkerResponseMessage::from_str(message).unwrap();
            match worker_message {
                WorkerResponseMessage::Initialized => {
                    info!("worker message: {}", worker_message.to_string());
                }
                WorkerResponseMessage::Invert
                | WorkerResponseMessage::BoxBlur
                | WorkerResponseMessage::Gamma
                | WorkerResponseMessage::DisplayOriginalImage
                | WorkerResponseMessage::SobelEdgeDetector => {
                    let image_data = {
                        let image_data = Uint8ClampedArray::new(
                            &Reflect::get(&message_event.data(), &JsValue::from_str("image_data"))
                                .unwrap()
                                .dyn_into::<ArrayBuffer>()
                                .unwrap(),
                        );
                        let width = Reflect::get(&message_event.data(), &JsValue::from_str("width"))
                            .unwrap()
                            .as_f64()
                            .unwrap() as u32;

                        log::debug!("image width: {}", width);
                        ImageData::new_with_u8_clamped_array(
                            wasm_bindgen::Clamped(&image_data.to_vec()),
                            width,
                        )
                        .unwrap()
                    };
                    let center_x =
                        Reflect::get(&message_event.data(), &JsValue::from_str("center_x"))
                            .unwrap()
                            .as_f64()
                            .unwrap();
                    let center_y =
                        Reflect::get(&message_event.data(), &JsValue::from_str("center_y"))
                            .unwrap()
                            .as_f64()
                            .unwrap();
                    // let selected_image = selected_image_canvas.get_untracked().unwrap();
                    let selected_image = selected_image_canvas.get().unwrap();

                    let canvas_context = selected_image
                        .get_context("2d")
                        .unwrap()
                        .unwrap()
                        .dyn_into::<CanvasRenderingContext2d>()
                        .unwrap();

                    log::debug!("{}", "worker output");
                    log::debug!("{}", selected_image.width());
                    log::debug!("{}", selected_image.height());
                    canvas_context.clear_rect(
                        0.0,
                        0.0,
                        selected_image.width() as f64,
                        selected_image.height() as f64,
                    );

                    canvas_context
                        .put_image_data(&image_data, center_x, center_y)
                        .unwrap();
                }
            }
        });

    let worker_options = WorkerOptions::new();
    worker_options.set_type(WorkerType::Module);
    // look into using Refcell like in the rustwasm example
    let worker =
        Rc::new(web_sys::Worker::new_with_options("./worker_loader.js", &worker_options).unwrap());
    worker.set_onmessage(Some(on_worker_message.as_ref().unchecked_ref()));
    on_worker_message.forget();

    worker.clone()
}

pub fn use_screen_width() -> ReadSignal<bool> {
    let query = "(min-width: 1024px)";
    let media_query = window().unwrap().match_media(query).unwrap().unwrap();
    let (is_screen_desktop_size, set_is_screen_desktop_size) = create_signal(media_query.matches());

    let on_screen_width_change: Closure<dyn FnMut(MediaQueryListEvent)> =
        Closure::new(move |event: MediaQueryListEvent| {
            // only gets called if the size changes from desktop to mobile or whatever i specified and vice versa
            info!("{:?}", event);
            set_is_screen_desktop_size.set(event.matches());
        });
    // put this in a create_effect, maybe?
    match media_query
        .add_event_listener_with_callback("change", on_screen_width_change.as_ref().unchecked_ref())
    {
        Ok(_) => on_screen_width_change.forget(),
        Err(_) => log::error!("error setting up listener to observe screen width changes"),
    }

    is_screen_desktop_size
}
