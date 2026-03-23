# Lumiavis

Lumiavis is a lightweight Rust library for camera capture, image processing, and simple video pipelines.

It provides a minimal, clean abstraction over:
- webcam capture (V4L2)
- image decoding (MJPEG → RGB)
- image operations (resize, crop, grayscale, draw)
- preview rendering
- video export (JPEG sequence → MP4 via ffmpeg)

---

## ✨ Features

- 📷 Camera capture (V4L2)
- 🖼️ Image decoding (MJPEG → RGB)
- 🧩 Image ops:
  - resize
  - crop
  - grayscale
  - draw rectangle
- 🖥️ Preview window (via examples)
- 🎞️ Capture to JPEG sequence
- 🎬 Export MP4 using ffmpeg

---

## 🚀 Quick Example

### Capture and Save One Frame

```rust
use lumiavis::{Camera, CameraConfig, FrameFormat, Resolution};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = CameraConfig::new(0)
        .with_resolution(Resolution::new(1280, 720))
        .with_fps(30)
        .with_format(FrameFormat::Mjpeg);

    let mut camera = Camera::open(config)?;
    let frame = camera.read_frame()?;

    frame.save("frame.jpg")?;

    Ok(())
}
```

---

### Capture Video (JPEG Sequence)

```rust
use lumiavis::{Camera, CameraConfig, CaptureSession, FrameFormat, Resolution};

let config = CameraConfig::new(0)
    .with_resolution(Resolution::new(1280, 720))
    .with_fps(30)
    .with_format(FrameFormat::Mjpeg);

let camera = Camera::open(config)?;
let mut session = CaptureSession::new(camera);

let stats = session.capture_to_jpeg_sequence("video_frames", 120)?;

println!("fps: {:.2}", stats.effective_fps);
```

---

### Export to MP4

Requires ffmpeg:

```bash
sudo apt install ffmpeg
```

```rust
use lumiavis::{export_jpeg_sequence_to_mp4, Mp4ExportOptions};

let options = Mp4ExportOptions::default();

export_jpeg_sequence_to_mp4(
    "video_frames",
    "output.mp4",
    &options,
)?;
```

---

## 📦 Architecture

Camera → Frame → decode → Image → ops → render/export

---

## ⚠️ Notes

- MJPEG decoding is CPU-heavy
- MP4 export uses external ffmpeg
- Preview is provided via examples

---

## License

MIT
