use crate::LumiavisError;
use crate::core::resolution::Resolution;
use crate::image::image::Image;
use crate::image::pixel_format::PixelFormat;

pub fn resize(image: &Image, new_size: Resolution) -> Result<Image, LumiavisError> {
    if new_size.width == 0 || new_size.height == 0 {
        return Err(LumiavisError::InvalidConfig(
            "resize width/height must be > 0".to_string(),
        ));
    }

    match image.pixel_format {
        PixelFormat::Rgb8 => {
            let buffer = image::RgbImage::from_raw(
                image.resolution.width,
                image.resolution.height,
                image.data.clone(),
            )
            .ok_or_else(|| {
                LumiavisError::BackendError("failed to create RGB image buffer".to_string())
            })?;

            let resized = image::imageops::resize(
                &buffer,
                new_size.width,
                new_size.height,
                image::imageops::FilterType::Triangle,
            );

            Ok(Image {
                resolution: new_size,
                pixel_format: PixelFormat::Rgb8,
                data: resized.into_raw(),
            })
        }
        PixelFormat::Rgba8 => {
            let buffer = image::RgbaImage::from_raw(
                image.resolution.width,
                image.resolution.height,
                image.data.clone(),
            )
            .ok_or_else(|| {
                LumiavisError::BackendError("failed to create RGBA image buffer".to_string())
            })?;

            let resized = image::imageops::resize(
                &buffer,
                new_size.width,
                new_size.height,
                image::imageops::FilterType::Triangle,
            );

            Ok(Image {
                resolution: new_size,
                pixel_format: PixelFormat::Rgba8,
                data: resized.into_raw(),
            })
        }
        PixelFormat::Gray8 => {
            let buffer = image::GrayImage::from_raw(
                image.resolution.width,
                image.resolution.height,
                image.data.clone(),
            )
            .ok_or_else(|| {
                LumiavisError::BackendError("failed to create Gray image buffer".to_string())
            })?;

            let resized = image::imageops::resize(
                &buffer,
                new_size.width,
                new_size.height,
                image::imageops::FilterType::Triangle,
            );

            Ok(Image {
                resolution: new_size,
                pixel_format: PixelFormat::Gray8,
                data: resized.into_raw(),
            })
        }
    }
}
