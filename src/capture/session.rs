use crate::{CamError, Camera, Frame};
use std::fs;
use std::path::Path;
use std::time::Instant;

use crate::video::export::{Mp4ExportOptions, export_jpeg_sequence_to_mp4};

#[derive(Debug, Clone)]
pub struct CaptureStats {
    pub frames_captured: usize,
    pub elapsed_seconds: f64,
    pub effective_fps: f64,
}

pub struct CaptureSession {
    camera: Camera,
}

impl CaptureSession {
    pub fn new(camera: Camera) -> Self {
        Self { camera }
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn read_frame(&mut self) -> Result<Frame, CamError> {
        self.camera.read_frame()
    }

    pub fn capture_to_jpeg_sequence<P: AsRef<Path>>(
        &mut self,
        output_dir: P,
        total_frames: usize,
    ) -> Result<CaptureStats, CamError> {
        let output_dir = output_dir.as_ref();

        fs::create_dir_all(output_dir)
            .map_err(|e| CamError::BackendError(format!("failed to create output dir: {e}")))?;

        let start = Instant::now();

        for i in 0..total_frames {
            let frame = self.read_frame()?;

            let filename = output_dir.join(format!("frame_{:05}.jpg", i));
            frame
                .save(&filename)
                .map_err(|e| CamError::BackendError(format!("failed to save frame: {e}")))?;
        }

        let elapsed = start.elapsed().as_secs_f64();
        let effective_fps = if elapsed > 0.0 {
            total_frames as f64 / elapsed
        } else {
            0.0
        };

        Ok(CaptureStats {
            frames_captured: total_frames,
            elapsed_seconds: elapsed,
            effective_fps,
        })
    }

    pub fn capture_to_jpeg_sequence_and_export_mp4<
        P: AsRef<std::path::Path>,
        Q: AsRef<std::path::Path>,
    >(
        &mut self,
        output_dir: P,
        total_frames: usize,
        output_mp4: Q,
        options: &Mp4ExportOptions,
    ) -> Result<CaptureStats, CamError> {
        let stats = self.capture_to_jpeg_sequence(&output_dir, total_frames)?;
        export_jpeg_sequence_to_mp4(output_dir, output_mp4, options)?;
        Ok(stats)
    }
}
