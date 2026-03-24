use lumiavis::Camera;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Scanning system for connected cameras...\n");

    // Try enumerating up to 10 potential camera indices
    let max_devices_to_check = 10;
    let mut found_any = false;

    for i in 0..max_devices_to_check {
        // Use the list_modes feature to probe the device at index `i`
        match Camera::list_modes(i) {
            Ok(modes) => {
                found_any = true;
                println!("--- Camera Device Index {} ---", i);
                println!("Supported Profiles ({} total):", modes.len());

                // Print all available resolutions and frame rates the camera exposes
                for (idx, mode) in modes.iter().enumerate() {
                    println!(
                        "  [{:02}] {}x{} ({:?}, {})",
                        idx,
                        mode.resolution.width,
                        mode.resolution.height,
                        mode.format,
                        mode.fourcc
                    );
                }
                println!();
            }
            // A DeviceNotFound error means there is no camera at this index
            Err(lumiavis::LumiavisError::DeviceNotFound) => {
                continue;
            }
            Err(e) => {
                // If the OS denied access or another error happened, log it and keep going
                println!("--- Camera Device Index {} ---", i);
                println!("Could not read capabilities: {:?}", e);
                println!();
            }
        }
    }

    if !found_any {
        println!("No cameras were found attached to this system.");
    }

    Ok(())
}
