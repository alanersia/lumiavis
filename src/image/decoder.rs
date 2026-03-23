use crate::core::frame::Frame;
use crate::core::frame_format::FrameFormat;
use crate::error::LumiavisError;
use crate::image::image::Image;
use crate::image::pixel_format::PixelFormat;

pub fn decode_frame(frame: &Frame) -> Result<Image, LumiavisError> {
    match frame.format {
        FrameFormat::Mjpeg => decode_mjpeg(frame),
        _ => Err(LumiavisError::UnsupportedFormat),
    }
}

fn decode_mjpeg(frame: &Frame) -> Result<Image, LumiavisError> {
    let dyn_img = image::load_from_memory(&frame.data)
        .map_err(|e| LumiavisError::BackendError(format!("jpeg decode failed: {e}")))?;

    let rgb = dyn_img.to_rgb8();
    let (width, height) = rgb.dimensions();

    Ok(Image {
        resolution: crate::core::resolution::Resolution::new(width, height),
        pixel_format: PixelFormat::Rgb8,
        data: rgb.into_raw(),
    })
}
