use lumiavis::{
    Annotation, Camera, CameraConfig, DrawRect, FrameFormat, Resolution, RgbColor,
    image_to_u32_buffer,
};
use minifb::{Key, Window, WindowOptions};
use std::time::Instant;

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
        "Lumiavis Preview - CV Overlay",
        width,
        height,
        WindowOptions::default(),
    )?;

    let mut display_buffer = vec![0u32; width * height];

    let annotations = vec![
        Annotation::new(
            DrawRect::new(100, 80, 200, 120),
            "PERSON",
            Some(0.92),
            RgbColor::GREEN,
        ),
        Annotation::new(
            DrawRect::new(340, 120, 120, 90),
            "CAR",
            Some(0.87),
            RgbColor::RED,
        ),
    ];

    let mut frame_count = 0u64;
    let mut fps_timer = Instant::now();
    let mut fps_value = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame = camera.read_frame()?;
        let mut image = frame.decode()?;

        image.draw_annotations(&annotations)?;
        image.draw_fps_overlay(fps_value, 8, 8, RgbColor::WHITE, Some(RgbColor::BLUE))?;

        image_to_u32_buffer(&image, &mut display_buffer);
        window.update_with_buffer(&display_buffer, width, height)?;

        frame_count += 1;
        let elapsed = fps_timer.elapsed().as_secs_f64();
        if elapsed >= 1.0 {
            fps_value = frame_count as f64 / elapsed;
            window.set_title(&format!("Lumiavis Preview - {:.2} FPS", fps_value));

            frame_count = 0;
            fps_timer = Instant::now();
        }
    }

    Ok(())
}
