# Video Export Guide

While Lumiavis is designed to capture singular continuous frames, it provides a high-level wrapper—`CaptureSession`—for recording sessions and writing them to MP4 videos. 

*Note: The MP4 export relies on having `ffmpeg` installed on the host system.*

## The Capture Session

To record a video, you must pass your open camera to a `CaptureSession`. A typical workflow involves dumping the captured series of frames to a temporary directory as a JPEG sequence.

```rust
use lumiavis::{Camera, CameraConfig, CaptureSession};

let mut camera = Camera::open(CameraConfig::best_quality(0))?;

// Wrap the camera in a session
let mut session = CaptureSession::new(camera);

// Capture 300 sequential frames and save them as JPEGs inside the "temp_video" folder
let stats = session.capture_to_jpeg_sequence("temp_video", 300)?;

println!("Recorded {} frames at {:.2} FPS", stats.frames_captured, stats.effective_fps);
```

## Exporting via FFmpeg

Once you have a directory filled with a JPEG sequence (e.g. `frame_0001.jpg`, `frame_0002.jpg`), you can invoke the Lumiavis export bridge.

This function automatically determines the source framerate to match the exported file's length to reality, and generates an `output.mp4`.

```rust
use lumiavis::{export_jpeg_sequence_to_mp4, Mp4ExportOptions};

// Configure encoding options (defaults to H264 via libx264)
let mut opts = Mp4ExportOptions::default();
// opts.framerate = Some(30);

export_jpeg_sequence_to_mp4(
    "temp_video",   // Source directory containing the JPEG sequence
    "output.mp4",   // Destination file
    &opts
)?;

println!("Video exported successfully!");
```
