use lumiavis::{Camera, CameraConfig, FrameFormat, Resolution, image_to_u32_buffer};
use minifb::{Key, Window, WindowOptions};
// use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = CameraConfig::new(0)
        .with_resolution(Resolution::new(640, 360))
        .with_fps(30)
        .with_format(FrameFormat::Mjpeg);

    let mut camera = Camera::open(config)?;

    let state = camera.state();
    let width = state.resolution.width as usize;
    let height = state.resolution.height as usize;

    let mut window = Window::new("Lumiavis Preview", width, height, WindowOptions::default())?;

    let mut display_buffer = vec![0u32; width * height];

    // let mut frame_count = 0u64;
    // let mut fps_timer = Instant::now();

    // let mut read_total = 0.0;
    // let mut decode_total = 0.0;
    // let mut convert_total = 0.0;
    // let mut render_total = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame = camera.read_frame()?;
        let image = frame.decode()?;
        let processed = image.grayscale()?;

        image_to_u32_buffer(&processed, &mut display_buffer);

        window.update_with_buffer(&display_buffer, width, height)?;
    }

    Ok(())
}
