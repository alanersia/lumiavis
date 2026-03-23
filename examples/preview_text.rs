use lumiavis::{Camera, CameraConfig, FrameFormat, Resolution, RgbColor, image_to_u32_buffer};
use minifb::{Key, Window, WindowOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = CameraConfig::new(0)
        .with_resolution(Resolution::new(640, 360))
        .with_fps(30)
        .with_format(FrameFormat::Mjpeg);

    let mut camera = Camera::open(config)?;
    let state = camera.state();

    let width = state.resolution.width as usize;
    let height = state.resolution.height as usize;

    let mut window = Window::new(
        "Lumiavis Preview - Text",
        width,
        height,
        WindowOptions::default(),
    )?;

    let mut display_buffer = vec![0u32; width * height];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame = camera.read_frame()?;
        let mut image = frame.decode()?;

        image.draw_text(16, 16, "FPS: 18.7", RgbColor::GREEN)?;
        image.draw_text(16, 32, "PERSON 0.92", RgbColor::RED)?;

        image_to_u32_buffer(&image, &mut display_buffer);
        window.update_with_buffer(&display_buffer, width, height)?;
    }

    Ok(())
}
