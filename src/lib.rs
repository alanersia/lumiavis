pub mod backends;
pub mod core;
pub mod error;
pub mod prelude;

pub use error::LumiavisError;

pub use core::camera_mode::CameraMode;
pub use core::camera_state::CameraState;
pub use core::capture::camera::Camera;
pub use core::capture::config::CameraConfig;
pub use core::capture::session::{CaptureSession, CaptureStats};
pub use core::frame::Frame;
pub use core::frame_format::FrameFormat;
pub use core::image::decoder::decode_frame;
pub use core::image::model::Image;
pub use core::image::ops::crop::CropRect;
pub use core::image::ops::draw::{DrawRect, RgbColor};
pub use core::image::pixel_format::PixelFormat;
pub use core::render::display::image_to_u32_buffer;
pub use core::resolution::Resolution;
pub use core::video::export::{export_jpeg_sequence_to_mp4, Mp4ExportOptions};
pub use core::vision::annotation::{
    draw_annotations, draw_fps_overlay, draw_label_box, Annotation,
};
