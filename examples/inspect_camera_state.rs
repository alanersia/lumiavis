use lumiavis::{Camera, CameraConfig, FrameFormat, Resolution};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = CameraConfig::new(0)
        .with_resolution(Resolution::new(1280, 720))
        .with_fps(30)
        .with_format(FrameFormat::Mjpeg);

    let mut camera = Camera::open(config)?;

    let state = camera.state();
    println!("actual format : {:?}", state.format);
    println!(
        "actual size   : {}x{}",
        state.resolution.width, state.resolution.height
    );
    println!("actual fps    : {}", state.fps);

    let frame = camera.read_frame()?;
    frame.save("inspect_frame.jpg")?;

    println!("saved inspect_frame.jpg");
    println!("frame bytes   : {}", frame.bytes_used);
    println!("sequence      : {}", frame.sequence);

    Ok(())
}
