use lumiavis::{Camera, CameraConfig, image_to_u32_buffer};
use minifb::{Key, Window, WindowOptions};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Configure the camera for the highest possible resolution and frame rate
    let config = CameraConfig::best_quality(0);

    // 2. Open the camera
    let mut camera = Camera::open(config)?;
    let state = camera.state();
    println!("Camera successfully opened!");
    println!("Negotiated State: {:?}", state);

    let width = state.resolution.width as usize;
    let height = state.resolution.height as usize;

    // 3. Create a Minifb window for display
    let mut window = Window::new(
        "Lumiavis - Camera Preview",
        width,
        height,
        WindowOptions::default(),
    )?;
    window.limit_update_rate(None); // Let the camera drive the loop, prevent minifb 60fps limit alias
    let mut display_buffer = vec![0u32; width * height];

    let mut fps_timer = Instant::now();
    let mut frame_count = 0;
    let mut current_fps = 0.0;

    println!("Press ESC to exit...");

    // 4. Capture and render loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Read the latest frame from the camera
        let frame = camera.read_frame()?;

        // Decode the frame (handles MJPEG extraction and YUV->RGB conversions automatically)
        let mut image = match frame.to_image() {
            Ok(img) => img,
            Err(e) => {
                eprintln!("Failed to decode frame: {:?}", e);
                continue;
            }
        };

        // Calculate actual FPS
        frame_count += 1;
        if fps_timer.elapsed().as_secs_f64() >= 1.0 {
            current_fps = frame_count as f64;
            frame_count = 0;
            fps_timer = Instant::now();
        }

        // Draw an FPS overlay directly onto the image
        let text_color = lumiavis::RgbColor { r: 0, g: 255, b: 0 };
        let bg_color = lumiavis::RgbColor { r: 0, g: 0, b: 0 };
        let _ = image.draw_fps_overlay(current_fps, 10, 10, text_color, Some(bg_color));

        // Convert the RGB Image to the u32 buffer format required by Minifb
        image_to_u32_buffer(&image, &mut display_buffer);

        // Render to the window
        window.update_with_buffer(&display_buffer, width, height)?;
    }

    Ok(())
}
