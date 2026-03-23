use lumiavis::{Camera, CameraConfig, CropRect, FrameFormat, Resolution};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = CameraConfig::new(0)
        .with_resolution(Resolution::new(1280, 720))
        .with_fps(30)
        .with_format(FrameFormat::Mjpeg);

    let mut camera = Camera::open(config)?;
    let frame = camera.read_frame()?;

    let image = frame.decode()?;
    image.save("debug_full.png")?;

    let rect = CropRect::new(100, 100, 400, 300);

    // crop dari implementation lu
    let cropped_custom = image.crop(rect)?;
    cropped_custom.save("debug_crop_custom.png")?;

    // crop referensi dari crate image
    let rgb = image::RgbImage::from_raw(
        image.resolution.width,
        image.resolution.height,
        image.data.clone(),
    )
    .ok_or("failed to build rgb buffer")?;

    let cropped_ref =
        image::imageops::crop_imm(&rgb, rect.x, rect.y, rect.width, rect.height).to_image();

    cropped_ref.save("debug_crop_ref.png")?;

    println!("saved:");
    println!("- debug_full.png");
    println!("- debug_crop_custom.png");
    println!("- debug_crop_ref.png");

    Ok(())
}
