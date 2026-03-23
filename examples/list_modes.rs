use lumiavis::Camera;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let modes = Camera::list_modes(0)?;

    println!("supported camera modes:");
    for mode in modes {
        println!(
            "- {:?} ({}) {}x{}",
            mode.format, mode.fourcc, mode.resolution.width, mode.resolution.height
        );
    }

    Ok(())
}
