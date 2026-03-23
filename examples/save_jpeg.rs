use lumiavis::{Camera, CameraConfig, FrameFormat, Resolution};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = CameraConfig::new(0)
        .with_resolution(Resolution::new(1920, 1080))
        .with_fps(30)
        .with_format(FrameFormat::Mjpeg);

    let mut camera = Camera::open(config)?;

    let frame = camera.read_frame()?;

    if !frame.is_mjpeg() {
        println!("warning: frame is not MJPEG, format={:?}", frame.format);
    }

    frame.save("frame.jpg")?;

    println!("saved frame.jpg");
    println!("format   : {:?}", frame.format);
    println!(
        "size     : {}x{}",
        frame.resolution.width, frame.resolution.height
    );
    println!("bytes    : {}", frame.bytes_used);
    println!("sequence : {}", frame.sequence);

    Ok(())
}
