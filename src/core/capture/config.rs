use crate::core::frame_format::FrameFormat;
use crate::core::resolution::Resolution;

#[derive(Debug, Clone)]
pub struct CameraConfig {
    pub index: usize,
    pub resolution: Resolution,
    pub fps: u32,
    pub format: FrameFormat,
}

impl CameraConfig {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            resolution: Resolution::new(640, 480),
            fps: 30,
            format: FrameFormat::Mjpeg,
        }
    }

    pub fn with_resolution(mut self, resolution: Resolution) -> Self {
        self.resolution = resolution;
        self
    }

    pub fn with_fps(mut self, fps: u32) -> Self {
        self.fps = fps;
        self
    }

    pub fn with_format(mut self, format: FrameFormat) -> Self {
        self.format = format;
        self
    }
}
