# Camera Capture Guide

Lumiavis provides a unified, cross-platform interface for capturing video streams from USB and integrated webcams. Under the hood, it abstracts away V4L2 on Linux and Media Foundation on Windows.

## Device Enumeration

If you aren't sure which camera index belongs to which device, or what formats your camera supports, you can use the `Camera::list_modes(index)` method.

```rust
use lumiavis::Camera;

match Camera::list_modes(0) {
    Ok(modes) => {
        for mode in modes {
            println!("{}x{} @ {}fps ({})", 
                mode.resolution.width, 
                mode.resolution.height, 
                mode.fps, 
                mode.fourcc
            );
        }
    }
    Err(e) => println!("Camera not found or inaccessible: {:?}", e),
}
```

## Opening a Camera

Lumiavis configures cameras through the `CameraConfig` struct. You can dictate the exact resolution and framerate you want:

```rust
let config = CameraConfig::new(0)
    .with_resolution(Resolution::new(1920, 1080))
    .with_fps(30)
    .with_format(FrameFormat::Mjpeg);

let mut camera = Camera::open(config)?;
```

If you don't care about the specifics and just want the **best possible picture**, use the `best_quality` helper. This tells the backend to aggressively negotiate for the highest native resolution and framerate the device supports.

```rust
let config = CameraConfig::best_quality(0);
let mut camera = Camera::open(config)?;
```

## Reading Frames

Frames are read as raw byte arrays directly from the sensor.

```rust
let frame = camera.read_frame()?;
println!("Received {} bytes with sequence {}", frame.bytes_used, frame.sequence);
```

By default, the `frame.data` remains in whatever format the camera natively emitted. To reliably process or display this data, you must decode it into an `Image` object.

```rust
let image = frame.to_image()?;
```

If the camera emitted MJPEG, `to_image()` delegates to internal logic to decode the JPEG payload. If the camera emitted raw YUV (like YUY2 or NV12), Lumiavis will perform an incredibly fast software conversion to `Rgb8`. Ensure you build with `--release` for complex uncompressed 1080p conversions to guarantee 30+ FPS.
