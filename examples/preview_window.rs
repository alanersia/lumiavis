use lumiavis::{image_to_u32_buffer, Camera, CameraConfig, FrameFormat, Resolution};
use minifb::{Key, Window, WindowOptions};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = CameraConfig::new(1)
        .with_resolution(Resolution::new(640, 360))
        .with_fps(30)
        .with_format(FrameFormat::Mjpeg);

    let mut camera = Camera::open(config)?;

    let state = camera.state();
    println!("Camera state: {:?}", state);

    let width = state.resolution.width as usize;
    let height = state.resolution.height as usize;

    let mut window = Window::new("Lumiavis Preview", width, height, WindowOptions::default())?;

    let mut display_buffer = vec![0u32; width * height];

    let mut frame_count = 0u64;
    let mut fps_timer = Instant::now();

    let mut read_total = 0.0;
    let mut decode_total = 0.0;
    let mut convert_total = 0.0;
    let mut render_total = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // ================= READ =================
        let t0 = Instant::now();
        let frame = camera.read_frame()?;
        read_total += t0.elapsed().as_secs_f64();

        // ================= DECODE (SAFE) =================
        let t1 = Instant::now();

        let image = match frame.to_image() {
            Ok(img) => img,
            Err(e) => {
                eprintln!("failed to process frame: {:?}", e);
                continue;
            }
        };

        decode_total += t1.elapsed().as_secs_f64();

        // ================= CONVERT =================
        let t2 = Instant::now();
        image_to_u32_buffer(&image, &mut display_buffer);
        convert_total += t2.elapsed().as_secs_f64();

        // ================= RENDER =================
        let t3 = Instant::now();
        window.update_with_buffer(&display_buffer, width, height)?;
        render_total += t3.elapsed().as_secs_f64();

        frame_count += 1;

        // ================= FPS =================
        let elapsed = fps_timer.elapsed().as_secs_f64();
        if elapsed >= 1.0 {
            let fps = frame_count as f64 / elapsed;

            let read_ms = (read_total / frame_count as f64) * 1000.0;
            let decode_ms = (decode_total / frame_count as f64) * 1000.0;
            let convert_ms = (convert_total / frame_count as f64) * 1000.0;
            let render_ms = (render_total / frame_count as f64) * 1000.0;

            println!(
                "fps={:.2} | read={:.2}ms decode={:.2}ms convert={:.2}ms render={:.2}ms",
                fps, read_ms, decode_ms, convert_ms, render_ms
            );

            window.set_title(&format!("Lumiavis Preview - {:.2} FPS", fps));

            frame_count = 0;
            fps_timer = Instant::now();
            read_total = 0.0;
            decode_total = 0.0;
            convert_total = 0.0;
            render_total = 0.0;
        }
    }

    Ok(())
}
