use crate::core::image::pixel_format::PixelFormat;
use crate::core::resolution::Resolution;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Image {
    pub resolution: Resolution,
    pub pixel_format: PixelFormat,
    pub data: Vec<u8>,
}

impl Image {
    pub fn new(resolution: Resolution, pixel_format: PixelFormat, data: Vec<u8>) -> Self {
        Self {
            resolution,
            pixel_format,
            data,
        }
    }

    /// Loads an image from the filesystem and converts it to a standard Lumiavis Image
    /// (Rgb8, Rgba8, or Gray8)
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, crate::LumiavisError> {
        let img = image::open(path).map_err(|e| crate::LumiavisError::BackendError(e.to_string()))?;

        let res = Resolution::new(img.width(), img.height());

        let (pixel_format, data) = match img {
            image::DynamicImage::ImageLuma8(luma) => (PixelFormat::Gray8, luma.into_raw()),
            image::DynamicImage::ImageRgba8(rgba) => (PixelFormat::Rgba8, rgba.into_raw()),
            _ => {
                let rgb = img.to_rgb8();
                (PixelFormat::Rgb8, rgb.into_raw())
            }
        };

        Ok(Self {
            resolution: res,
            pixel_format,
            data,
        })
    }

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
        crate::core::image::ops::resize::resize(self, new_size)
    }

    pub fn crop(
        &self,
        rect: crate::core::image::ops::crop::CropRect,
    ) -> Result<Self, crate::LumiavisError> {
        crate::core::image::ops::crop::crop(self, rect)
    }

    pub fn grayscale(&self) -> Result<Self, crate::LumiavisError> {
        crate::core::image::ops::grayscale::grayscale(self)
    }

    pub fn draw_rect(
        &mut self,
        rect: crate::core::image::ops::draw::DrawRect,
        color: crate::core::image::ops::draw::RgbColor,
        thickness: u32,
    ) -> Result<(), crate::LumiavisError> {
        crate::core::image::ops::draw::draw_rect(self, rect, color, thickness)
    }

    pub fn draw_text(
        &mut self,
        x: u32,
        y: u32,
        text: &str,
        color: crate::core::image::ops::draw::RgbColor,
    ) -> Result<(), crate::LumiavisError> {
        crate::core::image::ops::text::draw_text(self, x, y, text, color)
    }

    pub fn fill_rect(
        &mut self,
        rect: crate::core::image::ops::draw::DrawRect,
        color: crate::core::image::ops::draw::RgbColor,
    ) -> Result<(), crate::LumiavisError> {
        crate::core::image::ops::draw::fill_rect(self, rect, color)
    }

    pub fn draw_annotation(&mut self, ann: &crate::Annotation) -> Result<(), crate::LumiavisError> {
        crate::core::vision::annotation::draw_label_box(self, ann)
    }

    pub fn draw_annotations(
        &mut self,
        annotations: &[crate::Annotation],
    ) -> Result<(), crate::LumiavisError> {
        crate::draw_annotations(self, annotations)
    }

    pub fn draw_fps_overlay(
        &mut self,
        fps: f64,
        x: u32,
        y: u32,
        text_color: crate::RgbColor,
        background: Option<crate::RgbColor>,
    ) -> Result<(), crate::LumiavisError> {
        crate::draw_fps_overlay(self, fps, x, y, text_color, background)
    }
}
