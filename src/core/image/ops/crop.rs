use crate::core::image::model::Image;
use crate::core::resolution::Resolution;
use crate::LumiavisError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CropRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl CropRect {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

pub fn crop(image: &Image, rect: CropRect) -> Result<Image, LumiavisError> {
    if rect.width == 0 || rect.height == 0 {
        return Err(LumiavisError::InvalidConfig(
            "crop width/height must be > 0".to_string(),
        ));
    }

    if rect.x + rect.width > image.resolution.width
        || rect.y + rect.height > image.resolution.height
    {
        return Err(LumiavisError::InvalidConfig(
            "crop rect is out of bounds".to_string(),
        ));
    }

    let channels = match image.pixel_format {
        crate::PixelFormat::Rgb8 => 3,
        crate::PixelFormat::Rgba8 => 4,
        crate::PixelFormat::Gray8 => 1,
    };

    let src_width = image.resolution.width as usize;
    let rect_x = rect.x as usize;
    let rect_y = rect.y as usize;
    let rect_w = rect.width as usize;
    let rect_h = rect.height as usize;

    let mut out = Vec::with_capacity(rect_w * rect_h * channels);

    for row in 0..rect_h {
        let src_y = rect_y + row;
        let start = (src_y * src_width + rect_x) * channels;
        let end = start + rect_w * channels;
        out.extend_from_slice(&image.data[start..end]);
    }

    Ok(Image {
        resolution: Resolution::new(rect.width, rect.height),
        pixel_format: image.pixel_format,
        data: out,
    })
}
