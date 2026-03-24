use lumiavis::{Camera, CameraConfig, Resolution};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Configure for a specific 1080p-like resolution and open the camera
    let config = CameraConfig::new(1).with_resolution(Resolution::new(1920, 1080));
    let mut camera = Camera::open(config)?;
    
    println!("Camera opened at {:?}", camera.state().resolution);
    println!("Warming up camera sensor...");

    // 2. Read and discard a few initial frames to allow the camera's auto-exposure
    // and auto-white-balance algorithms to adjust to the environment lighting.
    for _ in 0..10 {
        let _ = camera.read_frame()?;
    }

    // 3. Capture the actual frame we want to save
    println!("Capturing photo...");
    let frame = camera.read_frame()?;

    // 4. Decode the frame into an Image object
    let image = frame.to_image()?;

    // 5. Save the image to disk as a JPEG
    let output_path = "captured_photo.jpg";
    image.save(output_path)?;

    println!("Success! Photo saved to {}", output_path);
    
    Ok(())
}
