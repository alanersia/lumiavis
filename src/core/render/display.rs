use crate::{Image, PixelFormat};
use rayon::prelude::*;

pub fn image_to_u32_buffer(image: &Image, dst: &mut [u32]) {
    match image.pixel_format {
        PixelFormat::Rgb8 => fill_rgb(image, dst),
        PixelFormat::Gray8 => fill_gray(image, dst),
        PixelFormat::Rgba8 => fill_rgba(image, dst),
    }
}

fn fill_rgb(image: &Image, dst: &mut [u32]) {
    dst.par_iter_mut()
        .zip(image.data.par_chunks_exact(3))
        .for_each(|(d, chunk)| {
            let r = chunk[0] as u32;
            let g = chunk[1] as u32;
            let b = chunk[2] as u32;
            *d = (r << 16) | (g << 8) | b;
        });
}

fn fill_gray(image: &Image, dst: &mut [u32]) {
    dst.par_iter_mut()
        .zip(image.data.par_iter())
        .for_each(|(d, &v)| {
            let x = v as u32;
            *d = (x << 16) | (x << 8) | x;
        });
}

fn fill_rgba(image: &Image, dst: &mut [u32]) {
    dst.par_iter_mut()
        .zip(image.data.par_chunks_exact(4))
        .for_each(|(d, chunk)| {
            let r = chunk[0] as u32;
            let g = chunk[1] as u32;
            let b = chunk[2] as u32;
            *d = (r << 16) | (g << 8) | b;
        });
}
