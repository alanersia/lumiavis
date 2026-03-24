use lumiavis::{Image, Resolution, RgbColor};
use lumiavis::core::image::ops::{crop::CropRect, draw::DrawRect};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Note: ensure you have an image file named "input.jpg" in the project root,
    // or change the path below to an image you want to process.
    let input_path = "input.jpg";
    let output_path = "processed_output.jpg";

    println!("Attempting to load {}", input_path);
    
    // 1. Load an image from disk using Lumiavis
    let image = match Image::load(input_path) {
        Ok(img) => img,
        Err(_) => {
            println!("Could not load '{}'. Returning early.", input_path);
            println!("To test this example, please capture a photo using 'cargo run --example save_image' and rename it to 'input.jpg'.");
            return Ok(());
        }
    };
    
    println!("Loaded image with resolution: {:?}", image.resolution);

    // 2. Resize the image down to 800x600 using Bilinear interpolation
    println!("Resizing image...");
    let resized = image.resize(Resolution::new(800, 600))?;

    // 3. Crop a 400x400 section from the center
    println!("Cropping image...");
    // Just a rough estimate for center based on 800x600 size
    let crop_rect = CropRect { x: 200, y: 100, width: 400, height: 400 };
    let cropped = resized.crop(crop_rect)?;

    // 4. Convert the image to Grayscale
    println!("Applying grayscale filter...");
    let mut final_image = cropped.grayscale()?;

    // 5. Draw a red rectangle border around the edge
    println!("Drawing annotations...");
    let border = DrawRect { x: 10, y: 10, width: 380, height: 380 };
    let red = RgbColor { r: 255, g: 0, b: 0 };
    final_image.draw_rect(border, red, 5)?;
    
    // 6. Draw some text
    let white = RgbColor { r: 255, g: 255, b: 255 };
    final_image.draw_text(20, 20, "Lumiavis Processing", white)?;

    // 7. Save the processed image back to disk
    final_image.save(output_path)?;
    println!("Success! Processed image saved to {}", output_path);

    Ok(())
}
