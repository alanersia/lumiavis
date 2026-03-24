use crate::core::camera_mode::CameraMode;
use crate::core::camera_state::CameraState;
use crate::core::capture::config::CameraConfig;
use crate::core::frame::Frame;
use crate::core::frame_format::FrameFormat;
use crate::core::resolution::Resolution;
use crate::error::LumiavisError;

use v4l::buffer::Type;
use v4l::format::Format;
use v4l::fraction::Fraction;
use v4l::io::mmap::Stream as MmapStream;
use v4l::io::traits::CaptureStream;
use v4l::video::Capture;
use v4l::{Device, FourCC};

pub struct V4l2Camera {
    stream: MmapStream<'static>,
    state: CameraState,
}

impl V4l2Camera {
    pub fn list_modes(index: usize) -> Result<Vec<CameraMode>, LumiavisError> {
        let dev = Device::new(index).map_err(|e| LumiavisError::DeviceOpenFailed(e.to_string()))?;

        let mut modes = Vec::new();

        let formats = dev
            .enum_formats()
            .map_err(|e| LumiavisError::BackendError(e.to_string()))?;

        for fmt_desc in formats {
            let frame_format = map_fourcc(fmt_desc.fourcc);
            let fourcc = fmt_desc.fourcc.str().unwrap_or("UNKNOWN").to_string();

            let sizes = dev
                .enum_framesizes(fmt_desc.fourcc)
                .map_err(|e| LumiavisError::BackendError(e.to_string()))?;

            for size in sizes {
                match size.size {
                    v4l::framesize::FrameSizeEnum::Discrete(discrete) => {
                        modes.push(CameraMode {
                            format: frame_format,
                            fourcc: fourcc.clone(),
                            resolution: Resolution::new(discrete.width, discrete.height),
                        });
                    }
                    v4l::framesize::FrameSizeEnum::Stepwise(stepwise) => {
                        modes.push(CameraMode {
                            format: frame_format,
                            fourcc: fourcc.clone(),
                            resolution: Resolution::new(stepwise.max_width, stepwise.max_height),
                        });
                    }
                }
            }
        }

        Ok(modes)
    }

    pub fn open(config: CameraConfig) -> Result<Self, LumiavisError> {
        validate_config(&config)?;

        let mut dev = Device::new(config.index)
            .map_err(|e| LumiavisError::DeviceOpenFailed(e.to_string()))?;

        apply_format(&mut dev, &config)?;
        apply_fps(&dev, config.fps)?;

        let actual_format = dev
            .format()
            .map_err(|e| LumiavisError::BackendError(e.to_string()))?;

        let actual_fps = read_actual_fps(&dev).unwrap_or(config.fps);

        let state = CameraState {
            resolution: Resolution::new(actual_format.width, actual_format.height),
            format: map_fourcc(actual_format.fourcc),
            fps: actual_fps,
        };

        let leaked_dev: &'static Device = Box::leak(Box::new(dev));

        let stream = MmapStream::with_buffers(leaked_dev, Type::VideoCapture, 4)
            .map_err(|e| LumiavisError::StreamStartFailed(e.to_string()))?;

        Ok(Self { stream, state })
    }

    pub fn read_frame(&mut self) -> Result<Frame, LumiavisError> {
        const MAX_ATTEMPTS: usize = 8;

        for _ in 0..MAX_ATTEMPTS {
            let (data, meta) = self
                .stream
                .next()
                .map_err(|e| LumiavisError::FrameReadFailed(e.to_string()))?;

            let used = meta.bytesused as usize;

            if used == 0 || used > data.len() {
                continue;
            }

            let frame_data = &data[..used];

            if matches!(self.state.format, FrameFormat::Mjpeg) && !is_valid_jpeg(frame_data) {
                continue;
            }

            return Ok(Frame {
                resolution: self.state.resolution,
                format: self.state.format,
                data: frame_data.to_vec(),
                bytes_used: used,
                sequence: meta.sequence as u64,
            });
        }

        Err(LumiavisError::BackendError(
            "failed to read a valid frame after multiple attempts".to_string(),
        ))
    }

    pub fn state(&self) -> CameraState {
        self.state
    }
}

fn validate_config(config: &CameraConfig) -> Result<(), LumiavisError> {
    if config.resolution.width == 0 || config.resolution.height == 0 {
        return Err(LumiavisError::InvalidConfig(
            "resolution width/height must be > 0".to_string(),
        ));
    }

    if config.fps == 0 {
        return Err(LumiavisError::InvalidConfig("fps must be > 0".to_string()));
    }

    Ok(())
}

fn apply_format(dev: &mut Device, config: &CameraConfig) -> Result<(), LumiavisError> {
    let fourcc = map_frame_format(config.format);

    let fmt = Format::new(config.resolution.width, config.resolution.height, fourcc);

    dev.set_format(&fmt)
        .map_err(|e| LumiavisError::ConfigApplyFailed(e.to_string()))?;

    Ok(())
}

fn apply_fps(dev: &Device, fps: u32) -> Result<(), LumiavisError> {
    let mut params = dev
        .params()
        .map_err(|e| LumiavisError::ConfigApplyFailed(e.to_string()))?;

    params.interval = Fraction::new(1, fps);

    dev.set_params(&params)
        .map_err(|e| LumiavisError::ConfigApplyFailed(e.to_string()))?;

    Ok(())
}

fn read_actual_fps(dev: &Device) -> Result<u32, LumiavisError> {
    let params = dev
        .params()
        .map_err(|e| LumiavisError::BackendError(e.to_string()))?;

    let numerator = params.interval.numerator;
    let denominator = params.interval.denominator;

    if numerator == 0 {
        return Ok(0);
    }

    Ok(denominator / numerator)
}

fn map_fourcc(fourcc: v4l::FourCC) -> FrameFormat {
    match fourcc.str().unwrap_or("UNKNOWN") {
        "MJPG" => FrameFormat::Mjpeg,
        "YUYV" => FrameFormat::Yuyv,
        "H264" => FrameFormat::H264,
        "NV12" => FrameFormat::Nv12,
        "RGB3" => FrameFormat::Rgb8,
        "GREY" => FrameFormat::Gray8,
        _ => FrameFormat::Unknown,
    }
}

fn map_frame_format(format: FrameFormat) -> FourCC {
    match format {
        FrameFormat::Mjpeg => FourCC::new(b"MJPG"),
        FrameFormat::Yuyv => FourCC::new(b"YUYV"),
        FrameFormat::H264 => FourCC::new(b"H264"),
        FrameFormat::Nv12 => FourCC::new(b"NV12"),
        FrameFormat::Rgb8 => FourCC::new(b"RGB3"),
        FrameFormat::Gray8 => FourCC::new(b"GREY"),
        FrameFormat::Unknown => FourCC::new(b"MJPG"),
    }
}

fn is_valid_jpeg(data: &[u8]) -> bool {
    if data.len() < 4 {
        return false;
    }

    let starts_ok = data[0] == 0xFF && data[1] == 0xD8;
    let ends_ok = data[data.len() - 2] == 0xFF && data[data.len() - 1] == 0xD9;

    starts_ok && ends_ok
}
