use std::rc::Rc;
use std::str::FromStr;

use js_sys::{ArrayBuffer, Reflect, Uint8ClampedArray};
use leptos::{create_signal, html::Canvas, NodeRef, ReadSignal, SignalSet, StoredValue};
use log::info;
use shared::WorkerResponseMessage;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{
    window, CanvasRenderingContext2d, Event, HtmlCanvasElement, ImageData, MediaQueryListEvent,
    MessageEvent, Worker, WorkerOptions, WorkerType,
};

pub fn use_resize(
    offscreen_canvas: StoredValue<Rc<HtmlCanvasElement>>,
    selected_image_canvas: NodeRef<Canvas>,
) {
    let resize_closure: Closure<dyn FnMut(Event)> = Closure::new(move |_event: Event| {
        log::debug!("resizing");
        let canvas = selected_image_canvas.get_untracked().unwrap();
        let offscreen_canvas = offscreen_canvas.get_value();

        // todo: maybe think about how to handle an offscreen canvas that is empty
        // should be a no op
        // right now the default canvas size for the offscreen canvas
        //  is 150 x 300 so when resizing this code
        // will just write empty pixels/white pixels to the visible canvas
        let offscreen_ctx = offscreen_canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let image_data = offscreen_ctx
            .get_image_data(
                0.,
                0.,
                offscreen_canvas.width() as f64,
                offscreen_canvas.height() as f64,
            )
            .unwrap()
            .dyn_into::<ImageData>()
            .unwrap();

        let (scaled_width, scaled_height) =
            get_scaled_image_buffer_for_canvas(&image_data, &selected_image_canvas);

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
            .draw_image_with_html_canvas_element_and_dw_and_dh(
                &offscreen_canvas,
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
pub fn use_worker(
    selected_image_canvas: NodeRef<Canvas>,
    offscreen_canvas: StoredValue<Rc<HtmlCanvasElement>>,
) -> Rc<Worker> {
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

                        ImageData::new_with_u8_clamped_array(
                            wasm_bindgen::Clamped(&image_data.to_vec()),
                            width,
                        )
                        .unwrap()
                    };
                    // remove center_x and y from messages
                    // let center_x =
                    //     Reflect::get(&message_event.data(), &JsValue::from_str("center_x"))
                    //         .unwrap()
                    //         .as_f64()
                    //         .unwrap();
                    // let center_y =
                    //     Reflect::get(&message_event.data(), &JsValue::from_str("center_y"))
                    //         .unwrap()
                    //         .as_f64()
                    //         .unwrap();
                    let selected_image = selected_image_canvas.get().unwrap();
                    let (scaled_width, scaled_height) =
                        get_scaled_image_buffer_for_canvas(&image_data, &selected_image_canvas);

                    let canvas_context = selected_image
                        .get_context("2d")
                        .unwrap()
                        .unwrap()
                        .dyn_into::<CanvasRenderingContext2d>()
                        .unwrap();

                    canvas_context.clear_rect(
                        0.0,
                        0.0,
                        selected_image.width() as f64,
                        selected_image.height() as f64,
                    );

                    let center_x = (selected_image.width() as f64 - scaled_width) / 2.;
                    let center_y = (selected_image.height() as f64 - scaled_height) / 2.;

                    let offscreen_canvas = offscreen_canvas.get_value();
                    offscreen_canvas.set_width(image_data.width());
                    offscreen_canvas.set_height(image_data.height());

                    let offscreen_ctx = offscreen_canvas
                        .get_context("2d")
                        .unwrap()
                        .unwrap()
                        .dyn_into::<CanvasRenderingContext2d>()
                        .unwrap();
                    offscreen_ctx.put_image_data(&image_data, 0., 0.).unwrap();

                    canvas_context
                        .draw_image_with_html_canvas_element_and_dw_and_dh(
                            &offscreen_canvas,
                            center_x,
                            center_y,
                            scaled_width,
                            scaled_height,
                        )
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

fn get_scaled_image_buffer_for_canvas(
    image_data: &ImageData,
    canvas: &NodeRef<Canvas>,
) -> (f64, f64) {
    let canvas = canvas.get().unwrap();
    let canvas_client_width = canvas.client_width() as f64;
    let canvas_client_height = canvas.client_height() as f64;
    let image_width = image_data.width() as f64;
    let image_height = image_data.height() as f64;
    log::debug!("{}", image_width);
    log::debug!("{}", image_height);

    let width_scale = canvas_client_width / image_width;
    let height_scale = canvas_client_height / image_height;
    let scale = if width_scale < height_scale {
        width_scale
    } else {
        height_scale
    };

    let (new_width, new_height) =
        if canvas_client_width < image_width || canvas_client_height < image_height {
            (
                (image_width * scale).round(),
                (image_height * scale).round(),
            )
        } else {
            (image_width, image_height)
        };

    (new_width, new_height)
}
