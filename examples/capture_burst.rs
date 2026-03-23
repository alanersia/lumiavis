use lumiavis::{Camera, CameraConfig, FrameFormat, Resolution};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = CameraConfig::new(0)
        .with_resolution(Resolution::new(1920, 1080))
        .with_fps(30)
        .with_format(FrameFormat::Mjpeg);

    let mut camera = Camera::open(config)?;

    let total_frames = 10;

    for i in 0..total_frames {
        let frame = camera.read_frame()?;

        let filename = format!("frame_{:04}.jpg", i);
        frame.save(&filename)?;

        println!(
            "saved {} | seq={} | bytes={}",
            filename, frame.sequence, frame.bytes_used
        );
    }

    println!("done capturing {} frames", total_frames);

    Ok(())
}
