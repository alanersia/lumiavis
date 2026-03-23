use crate::core::frame_format::FrameFormat;
use crate::core::resolution::Resolution;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CameraState {
    pub resolution: Resolution,
    pub format: FrameFormat,
    pub fps: u32,
}
