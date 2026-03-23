use crate::core::resolution::Resolution;
use crate::image::pixel_format::PixelFormat;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Image {
    pub resolution: Resolution,
    pub pixel_format: PixelFormat,
    pub data: Vec<u8>,
}

impl Image {
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), image::ImageError> {
        let color = match self.pixel_format {
            PixelFormat::Rgb8 => image::ColorType::Rgb8,
            PixelFormat::Rgba8 => image::ColorType::Rgba8,
            PixelFormat::Gray8 => image::ColorType::L8,
        };

        image::save_buffer(
            path,
            &self.data,
            self.resolution.width,
            self.resolution.height,
            color,
        )
    }

    pub fn resize(
        &self,
        new_size: crate::core::resolution::Resolution,
    ) -> Result<Self, crate::LumiavisError> {
        crate::image::ops::resize::resize(self, new_size)
    }

    pub fn crop(&self, rect: crate::image::ops::crop::CropRect) -> Result<Self, crate::LumiavisError> {
        crate::image::ops::crop::crop(self, rect)
    }

    pub fn grayscale(&self) -> Result<Self, crate::LumiavisError> {
        crate::image::ops::grayscale::grayscale(self)
    }

    pub fn draw_rect(
        &mut self,
        rect: crate::image::ops::draw::DrawRect,
        color: crate::image::ops::draw::RgbColor,
        thickness: u32,
    ) -> Result<(), crate::LumiavisError> {
        crate::image::ops::draw::draw_rect(self, rect, color, thickness)
    }
}
