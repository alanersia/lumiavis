use crate::core::frame_format::FrameFormat;
use crate::core::resolution::Resolution;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CameraMode {
    pub format: FrameFormat,
    pub fourcc: String,
    pub resolution: Resolution,
}
