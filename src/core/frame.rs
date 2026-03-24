use std::path::Path;

use crate::core::frame_format::FrameFormat;
use crate::core::image::decoder::decode_frame;
use crate::core::image::model::Image;
use crate::core::image::pixel_format::PixelFormat;
use crate::core::resolution::Resolution;
use crate::error::LumiavisError;

#[derive(Debug, Clone)]
pub struct Frame {
    pub resolution: Resolution,
    pub format: FrameFormat,
    pub data: Vec<u8>,
    pub bytes_used: usize,
    pub sequence: u64,
}

impl Frame {
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn is_mjpeg(&self) -> bool {
        matches!(self.format, FrameFormat::Mjpeg)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), std::io::Error> {
        std::fs::write(path, &self.data)
    }

    pub fn decode(&self) -> Result<Image, LumiavisError> {
        decode_frame(self)
    }

    pub fn to_image(&self) -> Result<Image, LumiavisError> {
        match self.format {
            FrameFormat::Mjpeg => {
                // reuse existing decoder
                self.decode()
            }

            FrameFormat::Rgb8 => Ok(Image {
                resolution: self.resolution.clone(),
                pixel_format: PixelFormat::Rgb8,
                data: self.data.clone(),
            }),

            _ => Err(LumiavisError::UnsupportedFormat),
        }
    }
}
