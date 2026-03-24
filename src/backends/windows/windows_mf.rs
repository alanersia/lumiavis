use windows::Win32::{
    Media::MediaFoundation::*,
    System::Com::{CoInitializeEx, COINIT_MULTITHREADED},
};

use super::media_utils::*;
use crate::backends::traits::CameraBackend;
use crate::core::{
    camera_mode::CameraMode,
    camera_state::CameraState,
    capture::config::CameraConfig,
    frame::Frame,
    image::format::{
        jpeg::is_valid_jpeg,
        yuv::{nv12_to_rgb24, yuy2_to_rgb24},
    },
};
use crate::error::LumiavisError;

pub struct MediaFoundationCameraBackend {
    reader: IMFSourceReader,
    state: CameraState,
    sequence: u64,
    native_fmt: NativePixelFormat,
}

impl CameraBackend for MediaFoundationCameraBackend {
    fn list_modes(_index: usize) -> Result<Vec<CameraMode>, LumiavisError> {
        Ok(vec![])
    }

    fn open(config: CameraConfig) -> Result<Self, LumiavisError> {
        unsafe {
            CoInitializeEx(None, COINIT_MULTITHREADED)
                .ok()
                .map_err(|e| LumiavisError::BackendError(format!("{:?}", e)))?;

            MFStartup(MF_VERSION, 0)
                .map_err(|e| LumiavisError::BackendError(format!("{:?}", e)))?;

            let source = get_camera_source(config.index as u32)?;

            // MF_SOURCE_READER_ENABLE_VIDEO_PROCESSING = TRUE tells the source
            // reader to automatically insert a Video Processor MFT so we can
            // request RGB24 output even from a YUY2/NV12 camera.
            let mut reader_attrs: Option<IMFAttributes> = None;
            MFCreateAttributes(&mut reader_attrs, 2)
                .map_err(|e| LumiavisError::BackendError(format!("{:?}", e)))?;
            let reader_attrs = reader_attrs
                .ok_or_else(|| LumiavisError::BackendError("null reader attributes".into()))?;
            reader_attrs
                .SetUINT32(&MF_SOURCE_READER_ENABLE_VIDEO_PROCESSING, 1)
                .map_err(|e| LumiavisError::BackendError(format!("{:?}", e)))?;
            reader_attrs
                .SetUINT32(&MF_READWRITE_ENABLE_HARDWARE_TRANSFORMS, 1)
                .map_err(|e| LumiavisError::BackendError(format!("{:?}", e)))?;

            let reader = MFCreateSourceReaderFromMediaSource(&source, Some(&reader_attrs))
                .map_err(|e| LumiavisError::BackendError(format!("{:?}", e)))?;

            let (negotiated_format, actual_resolution, native_fmt) =
                configure_format(&reader, &config.resolution, config.fps)?;
            println!(
                "Native: {:?} → Output: {:?}, resolution: {:?}",
                native_fmt, negotiated_format, actual_resolution
            );

            Ok(Self {
                reader,
                state: CameraState {
                    resolution: actual_resolution,
                    fps: config.fps,
                    format: negotiated_format,
                },
                sequence: 0,
                native_fmt,
            })
        }
    }

    fn read_frame(&mut self) -> Result<Frame, LumiavisError> {
        const MAX_ATTEMPTS: usize = 32;

        for _ in 0..MAX_ATTEMPTS {
            // None = transient stream event (e.g. MF_SOURCE_READERF_STREAMTICK); just retry
            let raw = match unsafe { read_sample(&self.reader)? } {
                Some(d) => d,
                None => continue,
            };

            if raw.is_empty() {
                continue;
            }

            if matches!(
                self.state.format,
                crate::core::frame_format::FrameFormat::Mjpeg
            ) && !is_valid_jpeg(&raw)
            {
                continue;
            }

            // Apply software YUV→RGB24 conversion when WMF auto-convert was unavailable
            let data = match self.native_fmt {
                NativePixelFormat::Yuy2 => {
                    let w = self.state.resolution.width as usize;
                    let h = self.state.resolution.height as usize;
                    yuy2_to_rgb24(&raw, w, h)
                }
                NativePixelFormat::Nv12 => {
                    let w = self.state.resolution.width as usize;
                    let h = self.state.resolution.height as usize;
                    nv12_to_rgb24(&raw, w, h)
                }
                _ => raw,
            };

            let used = data.len();
            self.sequence += 1;

            return Ok(Frame {
                resolution: self.state.resolution,
                format: self.state.format,
                data,
                bytes_used: used,
                sequence: self.sequence,
            });
        }

        Err(LumiavisError::BackendError(
            "failed to read valid frame after max attempts".into(),
        ))
    }

    fn state(&self) -> CameraState {
        self.state.clone()
    }
}
