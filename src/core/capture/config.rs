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

    /// Automatically negotiates the highest available resolution and frame rate
    pub fn best_quality(index: usize) -> Self {
        Self {
            index,
            resolution: Resolution::new(0, 0),
            fps: 0,
            format: FrameFormat::Mjpeg, // Backend will try Mjpeg -> Rgb24 -> YUY2 based on priority
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
