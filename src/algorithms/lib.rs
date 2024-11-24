use std::fmt;
use std::{fmt::Display, str::FromStr};

use js_sys::{Object, Reflect, Uint8ClampedArray};
use wasm_bindgen::{Clamped, JsValue};
pub mod algorithms;
pub enum Command {
    NewImage,
    Invert,
    BoxBlur,
    Gamma,
    SobelEdgeDetector,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            NEW_IMAGE => Ok(Self::NewImage),
            INVERT => Ok(Self::Invert),
            BOX_BLUR => Ok(Self::BoxBlur),
            GAMMA => Ok(Self::Gamma),
            SOBEL_EDGE_DETECTOR => Ok(Self::SobelEdgeDetector),
            _ => Err(format!("Unsupported/Unknown command: {}", s)),
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Command::NewImage => NEW_IMAGE,
            Command::Invert => INVERT,
            Command::BoxBlur => BOX_BLUR,
            Command::Gamma => GAMMA,
            Command::SobelEdgeDetector => SOBEL_EDGE_DETECTOR,
        };

        write!(f, "{}", str)
    }
}

const NEW_IMAGE: &str = "new image";
const INVERT: &str = "invert";
const BOX_BLUR: &str = "box blur";
const GAMMA: &str = "gamma";
const SOBEL_EDGE_DETECTOR: &str = "sobel edge detector";
const WORKER_INITIALLZED: &str = "worker has finished initializing";

pub trait ToJsObject {
    fn to_js_object(self) -> Object;
}

pub struct NewImageMessage {
    message: String,
    image_data: Clamped<Vec<u8>>,
    new_width: f64,
    new_height: f64,
}

impl NewImageMessage {
    pub fn new(
        message: String,
        image_data: Clamped<Vec<u8>>,
        new_width: f64,
        new_height: f64,
    ) -> NewImageMessage {
        NewImageMessage {
            message,
            image_data,
            new_width,
            new_height,
        }
    }

    pub fn js_clamped_uint8_array(&self) -> Uint8ClampedArray {
        Uint8ClampedArray::from(self.image_data.0.as_ref())
    }
}

impl ToJsObject for NewImageMessage {
    fn to_js_object(self) -> Object {
        let message = Object::new();
        let raw_data = Uint8ClampedArray::from(self.image_data.0.as_ref());
        Reflect::set(
            &message,
            &JsValue::from_str("image_data"),
            &raw_data.buffer(),
        )
        .unwrap();
        Reflect::set(
            &message,
            &JsValue::from_str("message"),
            &JsValue::from_str(&self.message),
        )
        .unwrap();
        Reflect::set(
            &message,
            &JsValue::from_str("new_width"),
            &JsValue::from_f64(self.new_width),
        )
        .unwrap();
        Reflect::set(
            &message,
            &JsValue::from_str("new_height"),
            &JsValue::from_f64(self.new_height),
        )
        .unwrap();
        message
    }
}

pub struct GammaMessage {
    message: String,
    gamma: f64,
}

impl GammaMessage {
    pub fn new(message: String, gamma: f64) -> GammaMessage {
        GammaMessage { message, gamma }
    }
}

impl ToJsObject for GammaMessage {
    fn to_js_object(self) -> Object {
        let message = Object::new();
        Reflect::set(
            &message,
            &JsValue::from_str("message"),
            &JsValue::from_str(&self.message),
        )
        .unwrap();
        Reflect::set(
            &message,
            &JsValue::from_str(Command::Gamma.to_string().as_ref()),
            &JsValue::from_f64(self.gamma),
        )
        .unwrap();
        message
    }
}

pub struct SobelEdgeDetectionMessage {
    message: String,
    threshold: u32,
}

impl SobelEdgeDetectionMessage {
    pub fn new(message: String, threshold: u32) -> SobelEdgeDetectionMessage {
        SobelEdgeDetectionMessage { message, threshold }
    }
}

impl ToJsObject for SobelEdgeDetectionMessage {
    fn to_js_object(self) -> Object {
        let message = Object::new();
        Reflect::set(
            &message,
            &JsValue::from_str("message"),
            &JsValue::from_str(&self.message),
        )
        .unwrap();
        Reflect::set(
            &message,
            &JsValue::from_str(Command::SobelEdgeDetector.to_string().as_ref()),
            &JsValue::from_f64(self.threshold as f64),
        )
        .unwrap();
        message
    }
}

pub struct BoxBlurMessage {
    message: String,
    kernel_size: u32,
}

impl BoxBlurMessage {
    pub fn new(message: String, kernel_size: u32) -> BoxBlurMessage {
        BoxBlurMessage {
            message,
            kernel_size,
        }
    }
}

impl ToJsObject for BoxBlurMessage {
    fn to_js_object(self) -> Object {
        let message = Object::new();
        Reflect::set(
            &message,
            &JsValue::from_str("message"),
            &JsValue::from_str(&self.message),
        )
        .unwrap();
        Reflect::set(
            &message,
            &JsValue::from_str(Command::BoxBlur.to_string().as_ref()),
            &JsValue::from_f64(self.kernel_size as f64),
        )
        .unwrap();
        message
    }
}

pub struct InvertMessage {
    message: String,
    invert: bool,
}

impl InvertMessage {
    pub fn new(message: String, invert: bool) -> InvertMessage {
        InvertMessage { message, invert }
    }
}

impl ToJsObject for InvertMessage {
    fn to_js_object(self) -> Object {
        let message = Object::new();
        Reflect::set(
            &message,
            &JsValue::from_str("message"),
            &JsValue::from_str(self.message.as_str()),
        )
        .unwrap();
        Reflect::set(
            &message,
            &JsValue::from_str(Command::Invert.to_string().as_ref()),
            &JsValue::from_bool(self.invert),
        )
        .unwrap();
        message
    }
}

pub enum WorkerResponseMessage {
    Initialized,
    DisplayOriginalImage,
    Invert,
    BoxBlur,
    Gamma,
    SobelEdgeDetector,
}

impl FromStr for WorkerResponseMessage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "worker has finished initializing" => Ok(Self::Initialized),
            "original image" => Ok(Self::DisplayOriginalImage),
            "invert" => Ok(Self::Invert),
            "box blur" => Ok(Self::BoxBlur),
            "gamma" => Ok(Self::Gamma),
            "sobel edge detector" => Ok(Self::SobelEdgeDetector),
            _ => Err(format!("Unsupported/Unknown command: {}", s)),
        }
    }
}

impl Display for WorkerResponseMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            WorkerResponseMessage::Initialized => WORKER_INITIALLZED,
            WorkerResponseMessage::Invert => INVERT,
            WorkerResponseMessage::BoxBlur => BOX_BLUR,
            WorkerResponseMessage::Gamma => GAMMA,
            WorkerResponseMessage::SobelEdgeDetector => SOBEL_EDGE_DETECTOR,
            WorkerResponseMessage::DisplayOriginalImage => "original image",
        };

        write!(f, "{}", str)
    }
}
