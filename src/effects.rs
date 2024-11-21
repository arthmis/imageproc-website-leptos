use std::rc::Rc;
use std::str::FromStr;

use js_sys::{ArrayBuffer, Reflect, Uint8ClampedArray};
use leptos::{create_signal, html::Canvas, NodeRef, ReadSignal, SignalSet, WriteSignal};
use log::info;
use shared::WorkerResponseMessage;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{
    window, CanvasRenderingContext2d, ImageData, MediaQueryListEvent, MessageEvent, Worker,
    WorkerOptions, WorkerType,
};

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
                    let selected_image = selected_image_canvas.get_untracked().unwrap();

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
