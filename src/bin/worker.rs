#![feature(lazy_cell)]
use std::fmt;
use std::{
    fmt::Display,
    str::FromStr,
    sync::{LazyLock, Mutex},
};

use image_processing::pixel_ops::invert_mut;
use shared::{algorithms, Command, WorkerMessage};

use js_sys::{Array, ArrayBuffer, Boolean, Number, Object, Reflect, Uint8ClampedArray};
use log::info;
use wasm_bindgen::{prelude::*, Clamped, JsCast};
use web_sys::{
    CanvasRenderingContext2d, DedicatedWorkerGlobalScope, ImageData, MessageEvent, OffscreenCanvas,
    OffscreenCanvasRenderingContext2d,
};

/// this unmodified image will be used to perform nondestructive image processing
/// evertime a new command comes in, this image will be cloned and then processed
static UNMODIFIED_IMAGE: LazyLock<Mutex<RawImage>> =
    LazyLock::new(|| Mutex::new(RawImage::new(Vec::new(), 0)));

#[derive(Clone, Debug)]
pub struct RawImage {
    /// an image has 4 components, red, green, blue, alpha each represented by one byte/one
    /// position in the array
    buffer: Vec<u8>,
    /// height is equal to
    width: u32,
}

impl RawImage {
    pub fn new(buffer: Vec<u8>, width: u32) -> RawImage {
        RawImage { buffer, width }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        (self.buffer.len() / self.width() as usize / 4) as u32
    }

    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    pub fn to_vec(self) -> Vec<u8> {
        self.buffer
    }
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    let scope = std::rc::Rc::new(DedicatedWorkerGlobalScope::from(JsValue::from(
        js_sys::global(),
    )));
    let scope_clone = scope.clone();

    let on_message = Closure::wrap(Box::new(move |msg: MessageEvent| {
        web_sys::console::log_1(&"Worker received message".into());

        let input_message = Reflect::get(&msg.data(), &JsValue::from_str("message"))
            .unwrap()
            .as_string()
            .unwrap();
        info!("{}", &input_message);
        let command = match Command::from_str(&input_message) {
            Ok(command) => command,
            Err(error) => {
                info!("{}", &error);
                scope_clone
                    .post_message(&JsValue::from_str(&error))
                    .unwrap();
                return;
            }
        };

        match command {
            Command::NewImage => {
                info!("{:?}", &msg.data());
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
                let image_data = Reflect::get(&msg.data(), &JsValue::from_str("image_data"))
                    .unwrap()
                    .dyn_into::<ArrayBuffer>()
                    .unwrap();
                let image_data = Uint8ClampedArray::new(&image_data).to_vec();
                *UNMODIFIED_IMAGE.lock().unwrap() =
                    RawImage::new(image_data.to_vec(), image_width as u32);
                info!("image origin: x: {},  y: {}", center_x, center_y);
                info!(
                    "image width: {}, image height: {}",
                    image_width, image_height
                );
                info!("image data: {:?}", image_data);
                info!("image buffer length: {:?}", image_data.len());
            }
            Command::Invert => {
                info!("{}", Command::Invert.to_string());
                info!("{:?}", &msg.data());
                let should_invert = Reflect::get(
                    &msg.data(),
                    &JsValue::from_str(&Command::Invert.to_string()),
                )
                .unwrap()
                .dyn_into::<Boolean>()
                .unwrap()
                .as_bool()
                .unwrap();
                info!("should invert: {:?}", should_invert);
                let (image, width, worker_message) = {
                    let image = (*UNMODIFIED_IMAGE.lock().unwrap()).clone();
                    if image.buffer().is_empty() {
                        info!("no image selected to perform image processing");
                        return;
                    }
                    info!("{:?}", &image);

                    let width = image.width();
                    if should_invert {
                        info!("inverting image");
                        (
                            algorithms::invert(image.to_vec(), width),
                            width,
                            WorkerMessage::Invert,
                        )
                    } else {
                        (image.to_vec(), width, WorkerMessage::DisplayOriginalImage)
                    }
                };
                let image = Uint8ClampedArray::from(image.as_ref());
                let mut output_message = Object::new();

                Reflect::set(
                    &output_message,
                    &JsValue::from_str("message"),
                    &JsValue::from_str(worker_message.to_string().as_ref()),
                )
                .unwrap();
                Reflect::set(
                    &output_message,
                    &JsValue::from_str("image_data"),
                    &image.buffer(),
                )
                .unwrap();
                Reflect::set(
                    &output_message,
                    &JsValue::from_str("width"),
                    &JsValue::from_f64(width as f64),
                )
                .unwrap();
                info!("{:?}", &output_message);
                let array: Array = Array::new();
                array.push(&image.buffer());

                scope_clone
                    .post_message_with_transfer(&output_message, &array)
                    .unwrap();
            }
            Command::BoxBlur => {
                info!("{}", Command::BoxBlur.to_string());
                info!("{:?}", &msg.data());
                let box_blur_value = Reflect::get(
                    &msg.data(),
                    &JsValue::from_str(&Command::BoxBlur.to_string()),
                )
                .unwrap()
                .dyn_into::<Number>()
                .unwrap()
                .as_f64()
                .unwrap();
                let box_blur_value = box_blur_value as u32;
                info!("box blur value: {}", box_blur_value);
                let (image, width) = {
                    let image = (*UNMODIFIED_IMAGE.lock().unwrap()).clone();
                    if image.buffer().is_empty() {
                        info!("no image selected to perform image processing");
                        return;
                    }
                    info!("{:?}", &image);
                    let width = image.width();
                    (
                        algorithms::box_blur(image.to_vec(), width, box_blur_value),
                        width,
                    )
                };
                let image = Uint8ClampedArray::from(image.as_ref());
                let mut output_message = Object::new();

                Reflect::set(
                    &output_message,
                    &JsValue::from_str("message"),
                    &JsValue::from_str(WorkerMessage::BoxBlur.to_string().as_ref()),
                )
                .unwrap();
                Reflect::set(
                    &output_message,
                    &JsValue::from_str("image_data"),
                    &image.buffer(),
                )
                .unwrap();
                Reflect::set(
                    &output_message,
                    &JsValue::from_str("width"),
                    &JsValue::from_f64(width as f64),
                )
                .unwrap();
                info!("{:?}", &output_message);
                let array: Array = Array::new();
                array.push(&image.buffer());

                scope_clone
                    .post_message_with_transfer(&output_message, &array)
                    .unwrap();
            }
            Command::Gamma => {
                info!("{}", Command::Gamma.to_string());
                info!("{:?}", &msg.data());
                let gamma_value =
                    Reflect::get(&msg.data(), &JsValue::from_str(&Command::Gamma.to_string()))
                        .unwrap()
                        .dyn_into::<Number>()
                        .unwrap()
                        .as_f64()
                        .unwrap();
                info!("gamma value: {}", gamma_value);
                let (image, width) = {
                    let image = (*UNMODIFIED_IMAGE.lock().unwrap()).clone();
                    if image.buffer().is_empty() {
                        info!("no image selected to perform image processing");
                        return;
                    }
                    info!("{:?}", &image);
                    let width = image.width();
                    (
                        algorithms::gamma_transform(image.to_vec(), width, gamma_value as f32),
                        width,
                    )
                };
                let image = Uint8ClampedArray::from(image.as_ref());
                let mut output_message = Object::new();

                Reflect::set(
                    &output_message,
                    &JsValue::from_str("message"),
                    &JsValue::from_str(WorkerMessage::Gamma.to_string().as_ref()),
                )
                .unwrap();
                Reflect::set(
                    &output_message,
                    &JsValue::from_str("image_data"),
                    &image.buffer(),
                )
                .unwrap();
                Reflect::set(
                    &output_message,
                    &JsValue::from_str("width"),
                    &JsValue::from_f64(width as f64),
                )
                .unwrap();
                info!("{:?}", &output_message);
                let array: Array = Array::new();
                array.push(&image.buffer());

                scope_clone
                    .post_message_with_transfer(&output_message, &array)
                    .unwrap();
            }
            Command::SobelEdgeDetector => todo!(), // let canvas_context = canvas
        }
    }) as Box<dyn Fn(MessageEvent)>);

    scope.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
    on_message.forget();

    let output_message = Object::new();
    Reflect::set(
        &output_message,
        &JsValue::from_str("message"),
        &JsValue::from_str(WorkerMessage::Initialized.to_string().as_ref()),
    )
    .unwrap();
    scope.post_message(&output_message).unwrap();
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
