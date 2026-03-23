use v4l::buffer::Type;
use v4l::io::mmap::Stream as MmapStream;
use v4l::io::traits::CaptureStream;
use v4l::prelude::*;
use v4l::video::Capture;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dev = Device::new(0)?;

    let caps = dev.query_caps()?;
    println!("driver      : {}", caps.driver);
    println!("card        : {}", caps.card);
    println!("bus         : {}", caps.bus);
    println!(
        "version     : {}.{}.{}",
        caps.version.0, caps.version.1, caps.version.2
    );
    println!("capabilities: {:?}", caps.capabilities);

    let fmt = dev.format()?;
    println!(
        "current format: {}x{} fourcc={:?}",
        fmt.width, fmt.height, fmt.fourcc
    );

    let mut stream = MmapStream::with_buffers(&dev, Type::VideoCapture, 4)?;
    let (data, meta) = stream.next()?;

    println!("frame bytes : {}", data.len());
    println!("sequence    : {}", meta.sequence);
    println!("timestamp   : {:?}", meta.timestamp);
    println!("first bytes: {:02X?}", &data[..16.min(data.len())]);

    Ok(())
}
