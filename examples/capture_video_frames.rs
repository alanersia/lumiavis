use lumiavis::{Camera, CameraConfig, CaptureSession, FrameFormat, Resolution};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = CameraConfig::new(0)
        .with_resolution(Resolution::new(1280, 720))
        .with_fps(30)
        .with_format(FrameFormat::Mjpeg);

    let camera = Camera::open(config)?;
    let mut session = CaptureSession::new(camera);

    let stats = session.capture_to_jpeg_sequence("video_frames", 120)?;

    println!("done");
    println!("captured {} frames", stats.frames_captured);
    println!("elapsed: {:.2} sec", stats.elapsed_seconds);
    println!("effective fps: {:.2}", stats.effective_fps);

    Ok(())
}
