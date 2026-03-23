use crate::core::frame_format::FrameFormat;
use crate::core::resolution::Resolution;

#[derive(Debug, Clone)]
pub struct Frame {
    pub resolution: Resolution,
    pub format: FrameFormat,
    pub data: Vec<u8>,
    pub bytes_used: usize,
    pub sequence: u64,
}
