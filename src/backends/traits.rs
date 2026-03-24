use crate::core::{
    camera_mode::CameraMode, camera_state::CameraState, capture::config::CameraConfig, frame::Frame,
};
use crate::error::LumiavisError;

pub(crate) trait CameraBackend {
    fn list_modes(index: usize) -> Result<Vec<CameraMode>, LumiavisError>
    where
        Self: Sized;

    fn open(config: CameraConfig) -> Result<Self, LumiavisError>
    where
        Self: Sized;

    fn read_frame(&mut self) -> Result<Frame, LumiavisError>;

    fn state(&self) -> CameraState;
}
