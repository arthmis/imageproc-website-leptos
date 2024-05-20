use std::fmt;
use std::{fmt::Display, str::FromStr};
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
