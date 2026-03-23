use lumiavis::{Camera, CameraConfig, FrameFormat, Resolution};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = CameraConfig::new(0)
        .with_resolution(Resolution::new(1280, 720))
        .with_fps(30)
        .with_format(FrameFormat::Mjpeg);

    let mut camera = Camera::open(config)?;
    let frame = camera.read_frame()?;

    let image = frame.decode()?;
    let gray = image.grayscale()?;

    gray.save("grayscale.png")?;

    println!("original format : {:?}", image.pixel_format);
    println!("gray format     : {:?}", gray.pixel_format);
    println!("saved grayscale.png");

    Ok(())
}
