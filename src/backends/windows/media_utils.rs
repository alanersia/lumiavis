use crate::core::frame_format::FrameFormat;
use crate::error::LumiavisError;
use windows::Win32::Media::MediaFoundation::*;

/// Tracks what raw pixel format the camera is actually delivering,
/// so read_frame can apply software conversion when needed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NativePixelFormat {
    Mjpeg,
    Rgb24,
    Yuy2,
    Nv12,
}

pub unsafe fn get_camera_source(index: u32) -> Result<IMFMediaSource, LumiavisError> {
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
pub unsafe fn configure_format(
    reader: &IMFSourceReader,
    requested_res: &crate::core::resolution::Resolution,
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
        priority: u32, // lower = better
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

        // 1. Resolution match
        // If requested 0x0, prioritize highest resolution (MAX pixels - actual pixels)
        // If requested specific size, heavily penalize mismatches
        let res_diff = if requested_res.width == 0 || requested_res.height == 0 {
            // max realistic pixels ~ 33_177_600 (8K); we subtract actual so larger = smaller penalty
            33_177_600u32.saturating_sub(width * height) / 10000 // scale down to fit in u32 priority
        } else if width == requested_res.width && height == requested_res.height {
            0
        } else {
            10000 // strong penalty for wrong size
        };

        // 2. FPS match
        // If requested 0, prioritize highest FPS
        let fps_diff = if requested_fps == 0 {
            1000u32.saturating_sub(fps) // 1000fps - actual = penalty (higher fps = lower penalty)
        } else {
            (fps as i32 - requested_fps as i32).unsigned_abs() as u32
        };

        let priority = (priority_base as u32) + res_diff + fps_diff;

        let better = best.as_ref().map_or(true, |b| priority < b.priority);
        if better {
            best = Some(NativeCandidate {
                subtype,
                width,
                height,
                fps,
                native_fmt,
                priority,
            });
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
        if set_output_type(
            reader,
            &MFVideoFormat_MJPG,
            best.width,
            best.height,
            best.fps,
        ) {
            return Ok((FrameFormat::Mjpeg, res, NativePixelFormat::Mjpeg));
        }
    }

    // For YUV formats: try requesting RGB24 output first
    // (MF_SOURCE_READER_ENABLE_VIDEO_PROCESSING allows WMF to auto-insert a converter)
    if set_output_type(
        reader,
        &MFVideoFormat_RGB24,
        best.width,
        best.height,
        best.fps,
    ) {
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
pub unsafe fn set_output_type(
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
        .SetCurrentMediaType(MF_SOURCE_READER_FIRST_VIDEO_STREAM.0 as u32, None, &mt)
        .is_ok()
}

pub unsafe fn read_sample(reader: &IMFSourceReader) -> Result<Option<Vec<u8>>, LumiavisError> {
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
