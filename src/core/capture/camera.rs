use crate::backends::v4l2::V4l2Camera;
use crate::core::camera_mode::CameraMode;
use crate::core::camera_state::CameraState;
use crate::core::capture::config::CameraConfig;
use crate::core::frame::Frame;
use crate::error::LumiavisError;

pub struct Camera {
    inner: V4l2Camera,
}

impl Camera {
    pub fn list_modes(index: usize) -> Result<Vec<CameraMode>, LumiavisError> {
        V4l2Camera::list_modes(index)
    }

    pub fn open(config: CameraConfig) -> Result<Self, LumiavisError> {
        let inner = V4l2Camera::open(config)?;
        Ok(Self { inner })
    }

    pub fn read_frame(&mut self) -> Result<Frame, LumiavisError> {
        self.inner.read_frame()
    }

    pub fn state(&self) -> CameraState {
        self.inner.state()
    }
}
