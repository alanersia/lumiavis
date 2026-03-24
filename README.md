<div align="center">
  <h1>📸 Lumiavis</h1>
  <p><strong>A lightweight, elegant Rust library for cross-platform camera capture and image processing.</strong></p>
  <p>
    <a href="#"><img alt="License" src="https://img.shields.io/badge/license-Apache%202.0-blue.svg" /></a>
    <a href="#"><img alt="Rust Version" src="https://img.shields.io/badge/rust-1.70%2B-orange.svg" /></a>
    <a href="examples/"><img alt="Examples" src="https://img.shields.io/badge/examples-4%20Showcases-brightgreen" /></a>
  </p>
</div>

---

Lumiavis provides a minimal and clean abstraction over **V4L2** (Linux) and **Media Foundation** (Windows) to capture high-quality webcam frames. Beyond simple capture, it includes a robust image processing pipeline to load, manipulate, and export images and video sequences effortlessly.

## ✨ Features

- **🎥 Cross-Platform Capture:** Automatically negotiates the best resolution and framerate without messy boilerplate.
- **🖼️ Image Processing:** Easily load images from disk, resize, crop, and apply grayscale filters.
- **🖌️ Annotations:** Draw geometric shapes, text, bounding boxes, and real-time FPS overlays.
- **🎞️ Video Export:** Capture a sequence of JPEGs and compile them into an MP4 video using our FFmpeg bridge.

---

## 🚀 Quick Start

### 1. Camera Capture

Lumiavis makes it incredibly simple to find your camera, negotiate the maximum quality, and capture a photo:

```rust
use lumiavis::{Camera, CameraConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Automatically configures the camera for its highest resolution and FPS
    let config = CameraConfig::best_quality(0);
    
    // Open the camera and read a frame
    let mut camera = Camera::open(config)?;
    let frame = camera.read_frame()?;
    
    // Decode and save to disk
    frame.to_image()?.save("webcam_photo.jpg")?;
    
    Ok(())
}
```

### 2. Image Processing

No webcam? No problem. Lumiavis can also serve as a standalone image manipulation tool:

```rust
use lumiavis::{Image, Resolution, RgbColor};
use lumiavis::core::image::ops::crop::CropRect;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load an image directly from disk
    let mut image = Image::load("input.jpg")?;

    // Perform a series of operations cleanly
    let processed = image
        .resize(Resolution::new(800, 600))?
        .crop(CropRect { x: 200, y: 150, width: 400, height: 300 })?
        .grayscale()?;

    processed.save("output_processed.jpg")?;
    
    Ok(())
}
```

---

## 📚 Documentation

Dive deeper into Lumiavis by reading our dedicated tutorials:

- [**Camera Capture Guide**](docs/camera_capture.md): Learn about device enumeration, hardware acceleration, and frame reading.
- [**Image Processing Guide**](docs/image_processing.md): Learn how the internal NV12/YUY2 to RGB conversions work, and how to apply annotations.
- [**Video Export Guide**](docs/video_export.md): Learn how to use `CaptureSession` to record JPEG sequences and export them to MP4.

---

## 💡 Examples

We provide heavily-commented, practical examples to get you started quickly. Run them using cargo:

```bash
cargo run --release --example camera_preview
cargo run --release --example save_image
cargo run --release --example image_processing
cargo run --release --example list_cameras
```
*(**Note:** High-resolution software pixel conversions are CPU-intensive. It is recommended to run camera examples in `--release` mode for a smooth 30+ FPS experience).*

---

## License

This project is licensed under the [Apache License 2.0](LICENSE).
