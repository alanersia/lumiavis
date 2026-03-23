use lumiavis::{Camera, CameraConfig, FrameFormat, Resolution};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = CameraConfig::new(0)
        .with_resolution(Resolution::new(1920, 1080))
        .with_fps(30)
        .with_format(FrameFormat::Mjpeg);

    let mut camera = Camera::open(config)?;

    for i in 0..30 {
        let frame = camera.read_frame()?;
        let filename = format!("stream_{:04}.jpg", i);
        frame.save(&filename)?;

        println!(
            "saved {} | seq={} | bytes={}",
            filename, frame.sequence, frame.bytes_used
        );

        thread::sleep(Duration::from_millis(33));
    }

    Ok(())
}
