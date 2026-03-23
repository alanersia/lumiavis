use lumiavis::{Camera, CameraConfig, FrameFormat, Resolution};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = CameraConfig::new(0)
        .with_resolution(Resolution::new(1280, 720))
        .with_fps(30)
        .with_format(FrameFormat::Mjpeg);

    let mut camera = Camera::open(config)?;
    let frame = camera.read_frame()?;

    let image = frame.decode()?;
    let resized = image.resize(Resolution::new(640, 360))?;

    resized.save("resized.png")?;

    println!(
        "original : {}x{}",
        image.resolution.width, image.resolution.height
    );
    println!(
        "resized  : {}x{}",
        resized.resolution.width, resized.resolution.height
    );
    println!("saved resized.png");

    Ok(())
}
