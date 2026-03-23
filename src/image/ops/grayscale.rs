use crate::LumiavisError;
use crate::image::image::Image;
use crate::image::pixel_format::PixelFormat;

pub fn grayscale(image: &Image) -> Result<Image, LumiavisError> {
    match image.pixel_format {
        PixelFormat::Gray8 => Ok(image.clone()),

        PixelFormat::Rgb8 => {
            let mut out =
                Vec::with_capacity((image.resolution.width * image.resolution.height) as usize);

            for chunk in image.data.chunks_exact(3) {
                let r = chunk[0] as f32;
                let g = chunk[1] as f32;
                let b = chunk[2] as f32;

                let gray = (0.299 * r + 0.587 * g + 0.114 * b).round() as u8;
                out.push(gray);
            }

            Ok(Image {
                resolution: image.resolution,
                pixel_format: PixelFormat::Gray8,
                data: out,
            })
        }

        PixelFormat::Rgba8 => {
            let mut out =
                Vec::with_capacity((image.resolution.width * image.resolution.height) as usize);

            for chunk in image.data.chunks_exact(4) {
                let r = chunk[0] as f32;
                let g = chunk[1] as f32;
                let b = chunk[2] as f32;

                let gray = (0.299 * r + 0.587 * g + 0.114 * b).round() as u8;
                out.push(gray);
            }

            Ok(Image {
                resolution: image.resolution,
                pixel_format: PixelFormat::Gray8,
                data: out,
            })
        }
    }
}
