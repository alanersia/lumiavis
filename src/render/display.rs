use crate::{Image, PixelFormat};

pub fn image_to_u32_buffer(image: &Image, dst: &mut [u32]) {
    match image.pixel_format {
        PixelFormat::Rgb8 => fill_rgb(image, dst),
        PixelFormat::Gray8 => fill_gray(image, dst),
        PixelFormat::Rgba8 => fill_rgba(image, dst),
    }
}

fn fill_rgb(image: &Image, dst: &mut [u32]) {
    for (i, chunk) in image.data.chunks_exact(3).enumerate() {
        let r = chunk[0] as u32;
        let g = chunk[1] as u32;
        let b = chunk[2] as u32;
        dst[i] = (r << 16) | (g << 8) | b;
    }
}

fn fill_gray(image: &Image, dst: &mut [u32]) {
    for (i, &v) in image.data.iter().enumerate() {
        let x = v as u32;
        dst[i] = (x << 16) | (x << 8) | x;
    }
}

fn fill_rgba(image: &Image, dst: &mut [u32]) {
    for (i, chunk) in image.data.chunks_exact(4).enumerate() {
        let r = chunk[0] as u32;
        let g = chunk[1] as u32;
        let b = chunk[2] as u32;
        dst[i] = (r << 16) | (g << 8) | b;
    }
}
