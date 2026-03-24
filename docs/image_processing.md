# Image Processing Guide

Lumiavis isn't just for cameras! The core `Image` struct is a powerful, standalone abstraction for generic image manipulation.

## Loading and Saving

You can easily read and write PNGs or JPEGs from disk:

```rust
use lumiavis::Image;

// Load an image (automatically maps to Grayscale, Rgb8, or Rgba8 depending on the source file)
let img = Image::load("my_photo.jpg")?;

// Do some processing...

// Save the image back out to disk
img.save("my_photo_processed.png")?;
```

## Resizing and Cropping

Lumiavis provides safe, chainable operators for altering image dimensions.

### Resizing
Resizing uses Bilinear interpolation to ensure smooth scaling:

```rust
use lumiavis::Resolution;
let smaller = image.resize(Resolution::new(640, 480))?;
```

### Cropping
You can crop regions of interest. Be careful to ensure your `CropRect` doesn't exceed the bounds of the original image, or you will receive an error.

```rust
use lumiavis::core::image::ops::crop::CropRect;

let rect = CropRect { x: 100, y: 100, width: 250, height: 250 };
let cropped = image.crop(rect)?;
```

## Colors and Annotations

You can draw directly onto an `Image`. This is particularly useful for overlaying AI bounding boxes, debug rects, or adding a heads-up display to a video feed.

### Drawing Shapes
```rust
use lumiavis::{RgbColor, core::image::ops::draw::DrawRect};

let red = RgbColor { r: 255, g: 0, b: 0 };
let r = DrawRect { x: 50, y: 50, width: 100, height: 100 };

// Draw a hollow rectangle with a line thickness of 3
image.draw_rect(r, red, 3)?;

// Fill a rectangle completely
image.fill_rect(r, red)?;
```

### Adding Text and Overlays
Lumiavis provides a specialized helper for drawing frames-per-second to an image, which is perfect for real-time webcam previews:

```rust
let green = RgbColor { r: 0, g: 255, b: 0 };
let black_bg = RgbColor { r: 0, g: 0, b: 0 };

// Draw "144.5 FPS" at coordinate (10, 10)
image.draw_fps_overlay(144.5, 10, 10, green, Some(black_bg))?;
```
