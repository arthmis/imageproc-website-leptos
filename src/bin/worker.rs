#![feature(lazy_cell)]
use std::fmt;
use std::{
    fmt::Display,
    str::FromStr,
    sync::{LazyLock, Mutex},
};

use shared::Command;

use js_sys::{Array, ArrayBuffer, Reflect, Uint8ClampedArray};
use log::info;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{
    CanvasRenderingContext2d, DedicatedWorkerGlobalScope, ImageData, MessageEvent, OffscreenCanvas,
    OffscreenCanvasRenderingContext2d,
};

/// this unmodified image will be used to perform nondestructive image processing
/// evertime a new command comes in, this image will be cloned and then processed
static UNMODIFIED_IMAGE: LazyLock<Mutex<Vec<u8>>> = LazyLock::new(|| Mutex::new(Vec::new()));

fn main() {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"worker starting".into());
    wasm_logger::init(wasm_logger::Config::default());

    let scope = DedicatedWorkerGlobalScope::from(JsValue::from(js_sys::global()));
    let scope_clone = scope.clone();

    scope
        .post_message(&JsValue::from_str("worker is initialized"))
        .unwrap();

    let onmessage = Closure::wrap(Box::new(move |msg: MessageEvent| {
        web_sys::console::log_1(&"got message".into());

        let message = Reflect::get(&msg.data(), &JsValue::from_str("message"))
            .unwrap()
            .as_string()
            .unwrap();
        let command = match Command::from_str(&message) {
            Ok(command) => command,
            Err(error) => {
                scope_clone
                    .post_message(&JsValue::from_str(&error))
                    .unwrap();
                return;
            }
        };

        match command {
            Command::NewImage => {
                info!("{:?}", &msg.data());
                let image_data = Reflect::get(&msg.data(), &JsValue::from_str("image_data"))
                    .unwrap()
                    .dyn_into::<ArrayBuffer>()
                    .unwrap();
                let image_data = Uint8ClampedArray::new(&image_data);
                *UNMODIFIED_IMAGE.lock().unwrap() = image_data.to_vec();
                let center_x = Reflect::get(&msg.data(), &JsValue::from_str("center_x"))
                    .unwrap()
                    .as_f64()
                    .unwrap();
                let center_y = Reflect::get(&msg.data(), &JsValue::from_str("center_y"))
                    .unwrap()
                    .as_f64()
                    .unwrap();
                let image_width = Reflect::get(&msg.data(), &JsValue::from_str("new_width"))
                    .unwrap()
                    .as_f64()
                    .unwrap();
                let image_height = Reflect::get(&msg.data(), &JsValue::from_str("new_height"))
                    .unwrap()
                    .as_f64()
                    .unwrap();
                info!("{} {}", center_x, center_y);
                info!("{} {}", image_width, image_height);
                info!("{:?}", image_data);
                info!("{:?}", image_data.length());

                let canvas = OffscreenCanvas::new(image_width as u32, image_height as u32).unwrap();
                //     .get_context("2d")
                //     .unwrap()
                //     .unwrap()
                //     .dyn_into::<OffscreenCanvasRenderingContext2d>()
                //     .unwrap();
                // let image_data = canvas_context
                //     .get_image_data(center_x, center_y, image_width, image_height)
                //     .unwrap();
                // info!("{:?}", image_data.data().0);
                // canvas_context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
                // let ((new_width, new_height), (center_x, center_y)) =
                //     resize_image_for_canvas(&image_node, &canvas);
                // need to clear canvas rect, if new image is smaller than the previous, or you will still see the old image
                // canvas_context
                //     .draw_image_with_html_image_element_and_dw_and_dh(
                //         &image_node,
                //         center_x,
                //         center_y,
                //         new_width,
                //         new_height,
                //     )
                //     .unwrap();
            }
            Command::Invert => todo!(),
            Command::BoxBlur => todo!(),
            Command::Gamma => todo!(),
            Command::SobelEdgeDetector => todo!(), // let canvas_context = canvas
        }
    }) as Box<dyn Fn(MessageEvent)>);
    scope.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();

    // The worker must send a message to indicate that it's ready to receive messages.
    // scope
    //     .post_message(&Array::new().into())
    //     .expect("posting ready message succeeds");
}

// fn resize_image_for_canvas(
//     image_node: &HtmlImageElement,
//     canvas: &OffscreenCanvas,
// ) -> ((f64, f64), (f64, f64)) {
//     let canvas_offset_width = canvas.width() as f64;
//     let canvas_offset_height = canvas.width() as f64;
//     let image_width = image_node.width() as f64;
//     let image_height = image_node.height() as f64;
//     let width_scale = canvas.width() as f64 / image_node.width() as f64;
//     let height_scale = canvas.width() as f64 / image_node.height() as f64;
//     let scale = if width_scale < height_scale {
//         width_scale
//     } else {
//         height_scale
//     };
//     let canvas_width_less_than_image_width = canvas_offset_width < image_width;
//     let canvas_height_less_than_image_height = canvas_offset_height < image_height;
//     let (new_width, new_height) =
//         if canvas_width_less_than_image_width || canvas_height_less_than_image_height {
//             (
//                 (image_width * scale).round(),
//                 (image_height * scale).round(),
//             )
//         } else {
//             (image_width, image_height)
//         };
//     let center_x = (canvas_offset_width as f64 - new_width) / 2.0;
//     let center_y = (canvas_offset_height as f64 - new_height) / 2.0;

//     ((new_width, new_height), (center_x, center_y))
// }
