use windows::Win32::{
    Media::MediaFoundation::*,
    System::Com::{CoInitializeEx, COINIT_MULTITHREADED},
};

use crate::core::{
    camera_mode::CameraMode, camera_state::CameraState, capture::config::CameraConfig,
    frame::Frame, frame_format::FrameFormat,
};
use crate::error::LumiavisError;

use crate::backends::traits::CameraBackend;

/// Tracks what raw pixel format the camera is actually delivering,
/// so read_frame can apply software conversion when needed.
#[derive(Debug, Clone, Copy, PartialEq)]
enum NativePixelFormat {
    Mjpeg,
    Rgb24,
    Yuy2,
    Nv12,
}

pub struct MediaFoundationCameraBackend {
    reader: IMFSourceReader,
    state: CameraState,
    sequence: u64,
    native_fmt: NativePixelFormat,
}

impl CameraBackend for MediaFoundationCameraBackend {
    fn list_modes(_index: usize) -> Result<Vec<CameraMode>, LumiavisError> {
        Ok(vec![])
    }

    fn open(config: CameraConfig) -> Result<Self, LumiavisError> {
        unsafe {
            CoInitializeEx(None, COINIT_MULTITHREADED)
                .ok()
                .map_err(|e| LumiavisError::BackendError(format!("{:?}", e)))?;

            MFStartup(MF_VERSION, 0)
                .map_err(|e| LumiavisError::BackendError(format!("{:?}", e)))?;

            let source = get_camera_source(config.index as u32)?;

            // MF_SOURCE_READER_ENABLE_VIDEO_PROCESSING = TRUE tells the source
            // reader to automatically insert a Video Processor MFT so we can
            // request RGB24 output even from a YUY2/NV12 camera.
            let mut reader_attrs: Option<IMFAttributes> = None;
            MFCreateAttributes(&mut reader_attrs, 2)
                .map_err(|e| LumiavisError::BackendError(format!("{:?}", e)))?;
            let reader_attrs = reader_attrs
                .ok_or_else(|| LumiavisError::BackendError("null reader attributes".into()))?;
            reader_attrs
                .SetUINT32(&MF_SOURCE_READER_ENABLE_VIDEO_PROCESSING, 1)
                .map_err(|e| LumiavisError::BackendError(format!("{:?}", e)))?;
            reader_attrs
                .SetUINT32(&MF_READWRITE_ENABLE_HARDWARE_TRANSFORMS, 1)
                .map_err(|e| LumiavisError::BackendError(format!("{:?}", e)))?;

            let reader = MFCreateSourceReaderFromMediaSource(&source, Some(&reader_attrs))
                .map_err(|e| LumiavisError::BackendError(format!("{:?}", e)))?;

            let (negotiated_format, actual_resolution, native_fmt) =
                configure_format(&reader, config.fps)?;
            println!(
                "Native: {:?} → Output: {:?}, resolution: {:?}",
                native_fmt, negotiated_format, actual_resolution
            );

            Ok(Self {
                reader,
                state: CameraState {
                    resolution: actual_resolution,
                    fps: config.fps,
                    format: negotiated_format,
                },
                sequence: 0,
                native_fmt,
            })
        }
    }

    fn read_frame(&mut self) -> Result<Frame, LumiavisError> {
        const MAX_ATTEMPTS: usize = 32;

        for _ in 0..MAX_ATTEMPTS {
            // None = transient stream event (e.g. MF_SOURCE_READERF_STREAMTICK); just retry
            let raw = match unsafe { read_sample(&self.reader)? } {
                Some(d) => d,
                None => continue,
            };

            if raw.is_empty() {
                continue;
            }

            if matches!(self.state.format, FrameFormat::Mjpeg) && !is_valid_jpeg(&raw) {
                continue;
            }

            // Apply software YUV→RGB24 conversion when WMF auto-convert was unavailable
            let data = match self.native_fmt {
                NativePixelFormat::Yuy2 => {
                    let w = self.state.resolution.width as usize;
                    let h = self.state.resolution.height as usize;
                    yuy2_to_rgb24(&raw, w, h)
                }
                NativePixelFormat::Nv12 => {
                    let w = self.state.resolution.width as usize;
                    let h = self.state.resolution.height as usize;
                    nv12_to_rgb24(&raw, w, h)
                }
                _ => raw,
            };

            let used = data.len();
            self.sequence += 1;

            return Ok(Frame {
                resolution: self.state.resolution,
                format: self.state.format,
                data,
                bytes_used: used,
                sequence: self.sequence,
            });
        }

        Err(LumiavisError::BackendError(
            "failed to read valid frame after max attempts".into(),
        ))
    }

    fn state(&self) -> CameraState {
        self.state.clone()
    }
}

//
// INTERNAL
//

unsafe fn get_camera_source(index: u32) -> Result<IMFMediaSource, LumiavisError> {
    let mut attributes: Option<IMFAttributes> = None;
    MFCreateAttributes(&mut attributes, 1)
        .map_err(|e| LumiavisError::BackendError(format!("{:?}", e)))?;
    let attributes =
        attributes.ok_or_else(|| LumiavisError::BackendError("null attributes".into()))?;

    attributes
        .SetGUID(
            &MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE,
            &MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_GUID,
        )
        .map_err(|e| LumiavisError::BackendError(format!("{:?}", e)))?;

    let mut pp_devices: *mut Option<IMFActivate> = std::ptr::null_mut();
    let mut count: u32 = 0;

    MFEnumDeviceSources(&attributes, &mut pp_devices, &mut count)
        .map_err(|e| LumiavisError::BackendError(format!("{:?}", e)))?;

    if index >= count {
        return Err(LumiavisError::BackendError("invalid device index".into()));
    }

    let devices = std::slice::from_raw_parts(pp_devices, count as usize);

    let device = devices[index as usize]
        .as_ref()
        .ok_or_else(|| LumiavisError::BackendError("null device".into()))?;

    let source: IMFMediaSource = device
        .ActivateObject()
        .map_err(|e| LumiavisError::BackendError(format!("{:?}", e)))?;

    Ok(source)
}

/// Enumerate the camera's native media types to find the best supported format,
/// then configure the source reader output type accordingly.
///
/// Returns (output FrameFormat, actual Resolution, native pixel format).
unsafe fn configure_format(
    reader: &IMFSourceReader,
    requested_fps: u32,
) -> Result<
    (
        FrameFormat,
        crate::core::resolution::Resolution,
        NativePixelFormat,
    ),
    LumiavisError,
> {
    let stream = MF_SOURCE_READER_FIRST_VIDEO_STREAM.0 as u32;

    struct NativeCandidate {
        subtype: windows::core::GUID,
        width: u32,
        height: u32,
        fps: u32,
        native_fmt: NativePixelFormat,
        priority: u8, // lower = better
    }

    let mut best: Option<NativeCandidate> = None;

    // Walk all native types and pick the highest-priority one
    let mut idx = 0u32;
    loop {
        let mt = match reader.GetNativeMediaType(stream, idx) {
            Ok(t) => t,
            Err(_) => break,
        };
        idx += 1;

        let Ok(subtype) = mt.GetGUID(&MF_MT_SUBTYPE) else {
            continue;
        };
        let Ok(packed) = mt.GetUINT64(&MF_MT_FRAME_SIZE) else {
            continue;
        };
        let width = (packed >> 32) as u32;
        let height = (packed & 0xFFFF_FFFF) as u32;

        let Ok(fps_packed) = mt.GetUINT64(&MF_MT_FRAME_RATE) else {
            continue;
        };
        let num = (fps_packed >> 32) as u32;
        let den = (fps_packed & 0xFFFF_FFFF) as u32;
        let fps = if den > 0 { num / den } else { 0 };

        let (priority_base, native_fmt) = if subtype == MFVideoFormat_MJPG {
            (0u8, NativePixelFormat::Mjpeg)
        } else if subtype == MFVideoFormat_NV12 {
            (10, NativePixelFormat::Nv12)
        } else if subtype == MFVideoFormat_YUY2 {
            (20, NativePixelFormat::Yuy2)
        } else {
            continue; // skip other formats
        };

        // Sub-priority based on FPS diff: closer to requested_fps = better priority
        let fps_diff = (fps as i32 - requested_fps as i32).unsigned_abs() as u8;
        let priority = priority_base + fps_diff;

        let better = best
            .as_ref()
            .map_or(true, |b| priority < b.priority);
        if better {
            best = Some(NativeCandidate { subtype, width, height, fps, native_fmt, priority });
        }
    }

    let best = best.ok_or_else(|| {
        LumiavisError::BackendError("No supported native format found on this camera".into())
    })?;

    let res = crate::core::resolution::Resolution {
        width: best.width,
        height: best.height,
    };

    // For MJPEG: request native MJPEG output directly
    if best.native_fmt == NativePixelFormat::Mjpeg {
        if set_output_type(reader, &MFVideoFormat_MJPG, best.width, best.height, best.fps) {
            return Ok((FrameFormat::Mjpeg, res, NativePixelFormat::Mjpeg));
        }
    }

    // For YUV formats: try requesting RGB24 output first
    // (MF_SOURCE_READER_ENABLE_VIDEO_PROCESSING allows WMF to auto-insert a converter)
    if set_output_type(reader, &MFVideoFormat_RGB24, best.width, best.height, best.fps) {
        return Ok((FrameFormat::Rgb8, res, NativePixelFormat::Rgb24));
    }

    // Fallback: set the native YUV output and do software conversion in read_frame
    if set_output_type(reader, &best.subtype, best.width, best.height, best.fps) {
        return Ok((FrameFormat::Rgb8, res, best.native_fmt));
    }

    Err(LumiavisError::BackendError(
        "Failed to set any output type on source reader".into(),
    ))
}

/// Set the source reader's output type to the given subtype + frame size.
/// Uses a fresh IMFMediaType each call to avoid 0xC00D5212.
unsafe fn set_output_type(
    reader: &IMFSourceReader,
    subtype: &windows::core::GUID,
    width: u32,
    height: u32,
    fps: u32,
) -> bool {
    let Ok(mt) = MFCreateMediaType() else {
        return false;
    };
    if mt.SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Video).is_err() {
        return false;
    }
    if mt.SetGUID(&MF_MT_SUBTYPE, subtype).is_err() {
        return false;
    }
    let size_packed: u64 = ((width as u64) << 32) | (height as u64);
    if mt.SetUINT64(&MF_MT_FRAME_SIZE, size_packed).is_err() {
        return false;
    }
    let fps_packed: u64 = ((fps as u64) << 32) | 1u64;
    if mt.SetUINT64(&MF_MT_FRAME_RATE, fps_packed).is_err() {
        return false;
    }
    reader
        .SetCurrentMediaType(
            MF_SOURCE_READER_FIRST_VIDEO_STREAM.0 as u32,
            None,
            &mt,
        )
        .is_ok()
}

unsafe fn read_sample(reader: &IMFSourceReader) -> Result<Option<Vec<u8>>, LumiavisError> {
    let mut stream_index = 0u32;
    let mut flags = 0u32;
    let mut timestamp = 0i64;
    let mut sample: Option<IMFSample> = None;

    reader
        .ReadSample(
            MF_SOURCE_READER_FIRST_VIDEO_STREAM.0 as u32,
            0,
            Some(&mut stream_index),
            Some(&mut flags),
            Some(&mut timestamp),
            Some(&mut sample),
        )
        .map_err(|e| LumiavisError::BackendError(format!("{:?}", e)))?;

    // A null sample is normal for stream events (stream tick, format change, etc.)
    let sample = match sample {
        Some(s) => s,
        None => return Ok(None),
    };

    let buffer = sample
        .ConvertToContiguousBuffer()
        .map_err(|e| LumiavisError::BackendError(format!("{:?}", e)))?;

    let mut ptr: *mut u8 = std::ptr::null_mut();
    let mut max_len: u32 = 0;
    let mut current_len: u32 = 0;

    buffer
        .Lock(&mut ptr, Some(&mut max_len), Some(&mut current_len))
        .map_err(|e| LumiavisError::BackendError(format!("{:?}", e)))?;

    let slice = std::slice::from_raw_parts(ptr, current_len as usize);
    let data = slice.to_vec();

    buffer
        .Unlock()
        .map_err(|e| LumiavisError::BackendError(format!("{:?}", e)))?;

    Ok(Some(data))
}

fn is_valid_jpeg(data: &[u8]) -> bool {
    data.len() >= 4
        && data[0] == 0xFF
        && data[1] == 0xD8
        && data[data.len() - 2] == 0xFF
        && data[data.len() - 1] == 0xD9
}

/// Software YUY2 (YUYV 4:2:2 packed) → RGB24 conversion.
fn yuy2_to_rgb24(src: &[u8], width: usize, height: usize) -> Vec<u8> {
    let pixels = width * height;
    let mut dst = vec![0u8; pixels * 3];
    let src = &src[..pixels * 2]; // clamp to expected byte count

    for (i, chunk) in src.chunks_exact(4).enumerate() {
        let y0 = chunk[0] as f32;
        let u  = chunk[1] as f32 - 128.0;
        let y1 = chunk[2] as f32;
        let v  = chunk[3] as f32 - 128.0;

        let conv = |y: f32| -> (u8, u8, u8) {
            let r = (y + 1.402 * v).clamp(0.0, 255.0) as u8;
            let g = (y - 0.344_136 * u - 0.714_136 * v).clamp(0.0, 255.0) as u8;
            let b = (y + 1.772 * u).clamp(0.0, 255.0) as u8;
            (r, g, b)
        };

        let (r0, g0, b0) = conv(y0);
        let (r1, g1, b1) = conv(y1);

        let p = i * 2;
        if p < pixels {
            dst[p * 3]     = r0;
            dst[p * 3 + 1] = g0;
            dst[p * 3 + 2] = b0;
        }
        if p + 1 < pixels {
            dst[(p + 1) * 3]     = r1;
            dst[(p + 1) * 3 + 1] = g1;
            dst[(p + 1) * 3 + 2] = b1;
        }
    }
    dst
}

/// Software NV12 (semi-planar YUV420) → RGB24 conversion.
fn nv12_to_rgb24(src: &[u8], width: usize, height: usize) -> Vec<u8> {
    let y_size = width * height;
    let mut dst = vec![0u8; y_size * 3];

    let y_plane = &src[..y_size.min(src.len())];
    let uv_plane = if src.len() > y_size { &src[y_size..] } else { &[] };

    for row in 0..height {
        for col in 0..width {
            let y = y_plane.get(row * width + col).copied().unwrap_or(16) as f32;
            let uv_row = row / 2;
            let uv_col = (col / 2) * 2;
            let u = uv_plane.get(uv_row * width + uv_col).copied().unwrap_or(128) as f32 - 128.0;
            let v = uv_plane.get(uv_row * width + uv_col + 1).copied().unwrap_or(128) as f32 - 128.0;

            let r = (y + 1.402 * v).clamp(0.0, 255.0) as u8;
            let g = (y - 0.344_136 * u - 0.714_136 * v).clamp(0.0, 255.0) as u8;
            let b = (y + 1.772 * u).clamp(0.0, 255.0) as u8;

            let p = row * width + col;
            dst[p * 3]     = r;
            dst[p * 3 + 1] = g;
            dst[p * 3 + 2] = b;
        }
    }
    dst
}
