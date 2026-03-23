use lumiavis::{Camera, CameraConfig, FrameFormat, Resolution};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = CameraConfig::new(0)
        .with_resolution(Resolution::new(1280, 720))
        .with_fps(30)
        .with_format(FrameFormat::Mjpeg);

    let mut camera = Camera::open(config)?;
    let frame = camera.read_frame()?;

    let image = frame.decode()?;
    image.save("decoded.png")?;

    println!("frame format   : {:?}", frame.format);
    println!(
        "image size     : {}x{}",
        image.resolution.width, image.resolution.height
    );
    println!("pixel format   : {:?}", image.pixel_format);
    println!("saved decoded.png");

    Ok(())
}
