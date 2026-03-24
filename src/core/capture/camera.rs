use crate::backends::traits::CameraBackend;
use crate::core::{
    camera_mode::CameraMode, camera_state::CameraState, capture::config::CameraConfig, frame::Frame,
};
use crate::error::LumiavisError;

#[cfg(target_os = "linux")]
use crate::backends::v4l2::V4l2CameraBackend;

#[cfg(target_os = "windows")]
use crate::backends::windows_mf::MediaFoundationCameraBackend;

pub struct Camera {
    inner: CameraInner,
}

enum CameraInner {
    #[cfg(target_os = "linux")]
    V4l2(V4l2CameraBackend),

    #[cfg(target_os = "windows")]
    Mf(MediaFoundationCameraBackend),
}

impl Camera {
    pub fn open(config: CameraConfig) -> Result<Self, LumiavisError> {
        #[cfg(target_os = "linux")]
        {
            let backend = V4l2CameraBackend::open(config)?;
            return Ok(Self {
                inner: CameraInner::V4l2(backend),
            });
        }

        #[cfg(target_os = "windows")]
        {
            let backend = MediaFoundationCameraBackend::open(config)?;
            return Ok(Self {
                inner: CameraInner::Mf(backend),
            });
        }

        #[allow(unreachable_code)]
        Err(LumiavisError::UnsupportedPlatform)
    }

    pub fn read_frame(&mut self) -> Result<Frame, LumiavisError> {
        match &mut self.inner {
            #[cfg(target_os = "linux")]
            CameraInner::V4l2(b) => b.read_frame(),

            #[cfg(target_os = "windows")]
            CameraInner::Mf(b) => b.read_frame(),
        }
    }

    pub fn state(&self) -> CameraState {
        match &self.inner {
            #[cfg(target_os = "linux")]
            CameraInner::V4l2(b) => b.state(),

            #[cfg(target_os = "windows")]
            CameraInner::Mf(b) => b.state(),
        }
    }

    pub fn list_modes(index: usize) -> Result<Vec<CameraMode>, LumiavisError> {
        #[cfg(target_os = "linux")]
        {
            return V4l2CameraBackend::list_modes(index);
        }

        #[cfg(target_os = "windows")]
        {
            return MediaFoundationCameraBackend::list_modes(index);
        }

        #[allow(unreachable_code)]
        Err(LumiavisError::UnsupportedPlatform)
    }
}
