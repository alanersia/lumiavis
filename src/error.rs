use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum LumiavisError {
    DeviceNotFound,
    DeviceOpenFailed(String),
    StreamStartFailed(String),
    FrameReadFailed(String),
    UnsupportedFormat,
    InvalidConfig(String),
    ConfigApplyFailed(String),
    BackendError(String),
}

impl Display for LumiavisError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DeviceNotFound => write!(f, "camera device not found"),
            Self::DeviceOpenFailed(msg) => write!(f, "failed to open device: {msg}"),
            Self::StreamStartFailed(msg) => write!(f, "failed to start stream: {msg}"),
            Self::FrameReadFailed(msg) => write!(f, "failed to read frame: {msg}"),
            Self::UnsupportedFormat => write!(f, "unsupported frame format"),
            Self::InvalidConfig(msg) => write!(f, "invalid camera config: {msg}"),
            Self::ConfigApplyFailed(msg) => write!(f, "failed to apply camera config: {msg}"),
            Self::BackendError(msg) => write!(f, "backend error: {msg}"),
        }
    }
}

impl std::error::Error for LumiavisError {}
