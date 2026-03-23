use crate::backends::v4l2::V4l2Camera;
use crate::capture::config::CameraConfig;
use crate::core::frame::Frame;
use crate::error::CamError;

pub struct Camera {
    inner: V4l2Camera,
}

impl Camera {
    pub fn open(config: CameraConfig) -> Result<Self, CamError> {
        let inner = V4l2Camera::open(config)?;
        Ok(Self { inner })
    }

    pub fn read_frame(&mut self) -> Result<Frame, CamError> {
        self.inner.read_frame()
    }
}
