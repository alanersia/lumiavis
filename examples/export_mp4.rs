use lumiavis::{Camera, CameraConfig, CaptureSession, FrameFormat, Mp4ExportOptions, Resolution};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = CameraConfig::new(0)
        .with_resolution(Resolution::new(1280, 720))
        .with_fps(30)
        .with_format(FrameFormat::Mjpeg);

    let camera = Camera::open(config)?;
    let mut session = CaptureSession::new(camera);

    let options = Mp4ExportOptions::default();

    let stats = session.capture_to_jpeg_sequence_and_export_mp4(
        "video_frames",
        120,
        "output.mp4",
        &options,
    )?;

    println!("captured {} frames", stats.frames_captured);
    println!("elapsed: {:.2} sec", stats.elapsed_seconds);
    println!("effective fps: {:.2}", stats.effective_fps);
    println!("saved output.mp4");

    Ok(())
}
