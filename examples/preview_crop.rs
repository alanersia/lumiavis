use lumiavis::{Camera, CameraConfig, CropRect, FrameFormat, Resolution, image_to_u32_buffer};
use minifb::{Key, Window, WindowOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = CameraConfig::new(0)
        .with_resolution(Resolution::new(640, 360))
        .with_fps(30)
        .with_format(FrameFormat::Mjpeg);

    let mut camera = Camera::open(config)?;

    let crop = CropRect::new(100, 50, 200, 150);

    let width = crop.width as usize;
    let height = crop.height as usize;

    let mut window = Window::new(
        "Lumiavis Preview - Crop",
        width,
        height,
        WindowOptions::default(),
    )?;

    let mut display_buffer = vec![0u32; width * height];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame = camera.read_frame()?;
        let image = frame.decode()?;
        let processed = image.crop(crop)?;

        image_to_u32_buffer(&processed, &mut display_buffer);
        window.update_with_buffer(&display_buffer, width, height)?;
    }

    Ok(())
}
