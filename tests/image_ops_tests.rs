use lumiavis::image::ops::crop::CropRect;
use lumiavis::{DrawRect, Image, LumiavisError, PixelFormat, Resolution, RgbColor};

fn make_rgb_image(width: u32, height: u32, data: Vec<u8>) -> Image {
    Image {
        resolution: Resolution::new(width, height),
        pixel_format: PixelFormat::Rgb8,
        data,
    }
}

#[test]
fn grayscale_converts_rgb8_to_gray8() -> Result<(), LumiavisError> {
    // 2 pixels:
    // pixel 1 = red   (255, 0, 0)
    // pixel 2 = green (0, 255, 0)
    let image = make_rgb_image(2, 1, vec![255, 0, 0, 0, 255, 0]);

    let gray = image.grayscale()?;

    assert_eq!(gray.pixel_format, PixelFormat::Gray8);
    assert_eq!(gray.resolution.width, 2);
    assert_eq!(gray.resolution.height, 1);
    assert_eq!(gray.data.len(), 2);

    // red grayscale ~= 76
    // green grayscale ~= 150
    assert_eq!(gray.data[0], 76);
    assert_eq!(gray.data[1], 150);

    Ok(())
}

#[test]
fn crop_returns_expected_size_and_data_length() -> Result<(), LumiavisError> {
    // 4x4 RGB image => 16 pixels => 48 bytes
    let data = vec![10u8; 4 * 4 * 3];
    let image = make_rgb_image(4, 4, data);

    let cropped = image.crop(CropRect::new(1, 1, 2, 2))?;

    assert_eq!(cropped.resolution.width, 2);
    assert_eq!(cropped.resolution.height, 2);
    assert_eq!(cropped.pixel_format, PixelFormat::Rgb8);
    assert_eq!(cropped.data.len(), 2 * 2 * 3);

    Ok(())
}

#[test]
fn resize_returns_expected_size() -> Result<(), LumiavisError> {
    // 4x4 RGB image => 48 bytes
    let data = vec![20u8; 4 * 4 * 3];
    let image = make_rgb_image(4, 4, data);

    let resized = image.resize(Resolution::new(2, 2))?;

    assert_eq!(resized.resolution.width, 2);
    assert_eq!(resized.resolution.height, 2);
    assert_eq!(resized.pixel_format, PixelFormat::Rgb8);
    assert_eq!(resized.data.len(), 2 * 2 * 3);

    Ok(())
}

#[test]
fn draw_rect_changes_boundary_pixels() -> Result<(), LumiavisError> {
    // 10x10 RGB black image
    let data = vec![0u8; 10 * 10 * 3];
    let mut image = make_rgb_image(10, 10, data);

    image.draw_rect(DrawRect::new(2, 2, 4, 3), RgbColor::RED, 1)?;

    // top-left boundary pixel of rectangle: (2,2)
    let idx = ((2usize * 10usize) + 2usize) * 3usize;

    assert_eq!(image.data[idx], 255); // R
    assert_eq!(image.data[idx + 1], 0); // G
    assert_eq!(image.data[idx + 2], 0); // B

    // outside pixel should remain black, example (0,0)
    let outside_idx = 0usize;
    assert_eq!(image.data[outside_idx], 0);
    assert_eq!(image.data[outside_idx + 1], 0);
    assert_eq!(image.data[outside_idx + 2], 0);

    Ok(())
}

#[test]
fn crop_out_of_bounds_returns_error() {
    let data = vec![0u8; 4 * 4 * 3];
    let image = make_rgb_image(4, 4, data);

    let result = image.crop(CropRect::new(3, 3, 2, 2));

    assert!(result.is_err());
}
