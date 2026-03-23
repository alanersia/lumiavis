use crate::core::image::model::Image;
use crate::core::image::pixel_format::PixelFormat;
use crate::LumiavisError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DrawRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl DrawRect {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbColor {
    pub const RED: Self = Self { r: 255, g: 0, b: 0 };
    pub const GREEN: Self = Self { r: 0, g: 255, b: 0 };
    pub const BLUE: Self = Self { r: 0, g: 0, b: 255 };
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
    };
}

pub fn draw_rect(
    image: &mut Image,
    rect: DrawRect,
    color: RgbColor,
    thickness: u32,
) -> Result<(), LumiavisError> {
    if rect.width == 0 || rect.height == 0 {
        return Err(LumiavisError::InvalidConfig(
            "draw rect width/height must be > 0".to_string(),
        ));
    }

    if thickness == 0 {
        return Err(LumiavisError::InvalidConfig(
            "draw rect thickness must be > 0".to_string(),
        ));
    }

    match image.pixel_format {
        PixelFormat::Rgb8 => draw_rect_rgb(image, rect, color, thickness),
        PixelFormat::Gray8 | PixelFormat::Rgba8 => Err(LumiavisError::UnsupportedFormat),
    }
}

pub fn fill_rect(image: &mut Image, rect: DrawRect, color: RgbColor) -> Result<(), LumiavisError> {
    if rect.width == 0 || rect.height == 0 {
        return Err(LumiavisError::InvalidConfig(
            "fill rect width/height must be > 0".to_string(),
        ));
    }

    match image.pixel_format {
        PixelFormat::Rgb8 => fill_rect_rgb(image, rect, color),
        PixelFormat::Gray8 | PixelFormat::Rgba8 => Err(LumiavisError::UnsupportedFormat),
    }
}

fn fill_rect_rgb(image: &mut Image, rect: DrawRect, color: RgbColor) -> Result<(), LumiavisError> {
    let img_w = image.resolution.width;
    let img_h = image.resolution.height;

    if rect.x >= img_w || rect.y >= img_h {
        return Err(LumiavisError::InvalidConfig(
            "fill rect origin out of bounds".to_string(),
        ));
    }

    let x2 = rect.x.saturating_add(rect.width).min(img_w);
    let y2 = rect.y.saturating_add(rect.height).min(img_h);

    for y in rect.y..y2 {
        for x in rect.x..x2 {
            set_rgb_pixel(image, x, y, color);
        }
    }

    Ok(())
}

fn draw_rect_rgb(
    image: &mut Image,
    rect: DrawRect,
    color: RgbColor,
    thickness: u32,
) -> Result<(), LumiavisError> {
    let img_w = image.resolution.width;
    let img_h = image.resolution.height;

    if rect.x >= img_w || rect.y >= img_h {
        return Err(LumiavisError::InvalidConfig(
            "draw rect origin out of bounds".to_string(),
        ));
    }

    let x2 = rect.x.saturating_add(rect.width).min(img_w);
    let y2 = rect.y.saturating_add(rect.height).min(img_h);

    for t in 0..thickness {
        let top_y = rect.y.saturating_add(t);
        let bottom_y = y2.saturating_sub(1).saturating_sub(t);
        let left_x = rect.x.saturating_add(t);
        let right_x = x2.saturating_sub(1).saturating_sub(t);

        if top_y < img_h {
            for x in left_x..x2 {
                set_rgb_pixel(image, x, top_y, color);
            }
        }

        if bottom_y < img_h {
            for x in left_x..x2 {
                set_rgb_pixel(image, x, bottom_y, color);
            }
        }

        if left_x < img_w {
            for y in rect.y..y2 {
                set_rgb_pixel(image, left_x, y, color);
            }
        }

        if right_x < img_w {
            for y in rect.y..y2 {
                set_rgb_pixel(image, right_x, y, color);
            }
        }
    }

    Ok(())
}

fn set_rgb_pixel(image: &mut Image, x: u32, y: u32, color: RgbColor) {
    let width = image.resolution.width as usize;
    let idx = ((y as usize * width) + x as usize) * 3;

    if idx + 2 < image.data.len() {
        image.data[idx] = color.r;
        image.data[idx + 1] = color.g;
        image.data[idx + 2] = color.b;
    }
}
