use crate::capture::config::CameraConfig;
use crate::core::frame::Frame;
use crate::core::frame_format::FrameFormat;
use crate::core::resolution::Resolution;
use crate::error::CamError;

use v4l::Device;
use v4l::buffer::Type;
use v4l::io::mmap::Stream as MmapStream;
use v4l::io::traits::CaptureStream;
use v4l::video::Capture;

pub struct V4l2Camera {
    dev: Device,
    _config: CameraConfig,
}

impl V4l2Camera {
    pub fn open(config: CameraConfig) -> Result<Self, CamError> {
        let dev =
            Device::new(config.index).map_err(|e| CamError::DeviceOpenFailed(e.to_string()))?;

        Ok(Self {
            dev,
            _config: config,
        })
    }

    pub fn read_frame(&mut self) -> Result<Frame, CamError> {
        let fmt = self
            .dev
            .format()
            .map_err(|e| CamError::BackendError(e.to_string()))?;

        let mut stream = MmapStream::with_buffers(&self.dev, Type::VideoCapture, 4)
            .map_err(|e| CamError::StreamStartFailed(e.to_string()))?;

        let (data, meta) = stream
            .next()
            .map_err(|e| CamError::FrameReadFailed(e.to_string()))?;

        Ok(Frame {
            resolution: Resolution::new(fmt.width, fmt.height),
            format: map_fourcc(fmt.fourcc),
            data: data.to_vec(),
            bytes_used: data.len(),
            sequence: meta.sequence as u64,
        })
    }
}

fn map_fourcc(fourcc: v4l::FourCC) -> FrameFormat {
    match fourcc.str().unwrap_or("UNKNOWN") {
        "MJPG" => FrameFormat::Mjpeg,
        "YUYV" => FrameFormat::Yuyv,
        _ => FrameFormat::Unknown,
    }
}
