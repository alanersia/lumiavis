use crate::core::frame::Frame;
use crate::core::frame_format::FrameFormat;
use crate::core::image::model::Image;
use crate::core::image::pixel_format::PixelFormat;
use crate::error::LumiavisError;

pub fn decode_frame(frame: &Frame) -> Result<Image, LumiavisError> {
    match frame.format {
        FrameFormat::Mjpeg => decode_mjpeg(frame),
        _ => Err(LumiavisError::UnsupportedFormat),
    }
}

fn decode_mjpeg(frame: &Frame) -> Result<Image, LumiavisError> {
    let mut decoder = zune_jpeg::JpegDecoder::new(std::io::Cursor::new(&frame.data[..]));
    let pixels = decoder.decode()
        .map_err(|e| LumiavisError::BackendError(format!("jpeg decode failed: {e}")))?;

    let info = decoder.info()
        .ok_or_else(|| LumiavisError::BackendError("failed to get jpeg info".to_string()))?;

    Ok(Image {
        resolution: crate::core::resolution::Resolution::new(info.width as u32, info.height as u32),
        pixel_format: PixelFormat::Rgb8,
        data: pixels,
    })
}
