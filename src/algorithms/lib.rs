use std::fmt;
use std::{fmt::Display, str::FromStr};
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

pub enum WorkerMessage {
    Initialized,
    DisplayOriginalImage,
    Invert,
    BoxBlur,
    Gamma,
    SobelEdgeDetector,
}

impl FromStr for WorkerMessage {
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

impl Display for WorkerMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            WorkerMessage::Initialized => WORKER_INITIALLZED,
            WorkerMessage::Invert => INVERT,
            WorkerMessage::BoxBlur => BOX_BLUR,
            WorkerMessage::Gamma => GAMMA,
            WorkerMessage::SobelEdgeDetector => SOBEL_EDGE_DETECTOR,
            WorkerMessage::DisplayOriginalImage => "original image",
        };

        write!(f, "{}", str)
    }
}
