#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameFormat {
    Mjpeg,
    Yuyv,
    H264,
    Nv12,
    Rgb8,
    Gray8,
    Unknown,
}
