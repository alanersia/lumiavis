use crate::CamError;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct Mp4ExportOptions {
    pub fps: u32,
    pub crf: u8,
    pub preset: String,
    pub overwrite: bool,
}

impl Default for Mp4ExportOptions {
    fn default() -> Self {
        Self {
            fps: 30,
            crf: 23,
            preset: "medium".to_string(),
            overwrite: true,
        }
    }
}

pub fn export_jpeg_sequence_to_mp4<P: AsRef<Path>, Q: AsRef<Path>>(
    input_dir: P,
    output_file: Q,
    options: &Mp4ExportOptions,
) -> Result<(), CamError> {
    let input_dir = input_dir.as_ref();
    let output_file = output_file.as_ref();

    let input_pattern = input_dir.join("frame_%05d.jpg");

    let mut cmd = Command::new("ffmpeg");

    if options.overwrite {
        cmd.arg("-y");
    } else {
        cmd.arg("-n");
    }

    cmd.args([
        "-framerate",
        &options.fps.to_string(),
        "-i",
        input_pattern
            .to_str()
            .ok_or_else(|| CamError::BackendError("invalid input pattern path".to_string()))?,
        "-c:v",
        "libx264",
        "-preset",
        &options.preset,
        "-crf",
        &options.crf.to_string(),
        "-pix_fmt",
        "yuv420p",
    ]);

    cmd.arg(
        output_file
            .to_str()
            .ok_or_else(|| CamError::BackendError("invalid output file path".to_string()))?,
    );

    let output = cmd
        .output()
        .map_err(|e| CamError::BackendError(format!("failed to run ffmpeg: {e}")))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(CamError::BackendError(format!(
            "ffmpeg failed: {}",
            stderr.trim()
        )));
    }

    Ok(())
}
