use lumiavis::core::image::ops::crop::CropRect;
use lumiavis::{Annotation, DrawRect, Image, LumiavisError, PixelFormat, Resolution, RgbColor};

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

#[test]
fn draw_text_modifies_image() -> Result<(), LumiavisError> {
    let mut image = Image::new(
        Resolution::new(32, 16),
        PixelFormat::Rgb8,
        vec![0u8; 32 * 16 * 3],
    );

    image.draw_text(0, 0, "A", RgbColor::WHITE)?;

    assert!(image.data.iter().any(|&v| v != 0));

    Ok(())
}

#[test]
fn draw_annotation_modifies_image() -> Result<(), LumiavisError> {
    let mut image = Image::new(
        Resolution::new(64, 32),
        PixelFormat::Rgb8,
        vec![0u8; 64 * 32 * 3],
    );

    let ann = Annotation::new(
        DrawRect::new(10, 10, 20, 10),
        "TEST",
        Some(0.99),
        RgbColor::GREEN,
    );

    image.draw_annotation(&ann)?;

    assert!(image.data.iter().any(|&v| v != 0));

    Ok(())
}

#[test]
fn fill_rect_modifies_image() -> Result<(), LumiavisError> {
    let mut image = Image::new(
        Resolution::new(16, 16),
        PixelFormat::Rgb8,
        vec![0u8; 16 * 16 * 3],
    );

    image.fill_rect(DrawRect::new(2, 2, 4, 4), RgbColor::GREEN)?;

    assert!(image.data.iter().any(|&v| v != 0));
    Ok(())
}

#[test]
fn draw_annotations_modifies_image() -> Result<(), LumiavisError> {
    let mut image = Image::new(
        Resolution::new(64, 32),
        PixelFormat::Rgb8,
        vec![0u8; 64 * 32 * 3],
    );

    let anns = vec![Annotation::new(
        DrawRect::new(10, 10, 20, 10),
        "TEST",
        Some(0.99),
        RgbColor::GREEN,
    )];

    image.draw_annotations(&anns)?;

    assert!(image.data.iter().any(|&v| v != 0));
    Ok(())
}
