use lumiavis::{Camera, CameraConfig, CropRect, FrameFormat, Resolution};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = CameraConfig::new(0)
        .with_resolution(Resolution::new(1280, 720))
        .with_fps(30)
        .with_format(FrameFormat::Mjpeg);

    let mut camera = Camera::open(config)?;
    let frame = camera.read_frame()?;

    let image = frame.decode()?;
    let cropped = image.crop(CropRect::new(100, 100, 400, 300))?;

    cropped.save("cropped.png")?;

    println!(
        "original : {}x{}",
        image.resolution.width, image.resolution.height
    );
    println!(
        "cropped  : {}x{}",
        cropped.resolution.width, cropped.resolution.height
    );
    println!("saved cropped.png");

    Ok(())
}
