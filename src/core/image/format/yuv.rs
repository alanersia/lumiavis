/// Software YUY2 (YUV 4:2:2) → RGB24 conversion.
pub fn yuy2_to_rgb24(src: &[u8], width: usize, height: usize) -> Vec<u8> {
    let pixels = width * height;
    let mut dst = vec![0u8; pixels * 3];

    for p in (0..pixels).step_by(2) {
        let i = p * 2;
        if i + 3 >= src.len() {
            break;
        }

        let y0 = src[i] as f32;
        let u = src[i + 1] as f32 - 128.0;
        let y1 = src[i + 2] as f32;
        let v = src[i + 3] as f32 - 128.0;

        let r0 = (y0 + 1.402 * v).clamp(0.0, 255.0) as u8;
        let g0 = (y0 - 0.344_136 * u - 0.714_136 * v).clamp(0.0, 255.0) as u8;
        let b0 = (y0 + 1.772 * u).clamp(0.0, 255.0) as u8;

        let r1 = (y1 + 1.402 * v).clamp(0.0, 255.0) as u8;
        let g1 = (y1 - 0.344_136 * u - 0.714_136 * v).clamp(0.0, 255.0) as u8;
        let b1 = (y1 + 1.772 * u).clamp(0.0, 255.0) as u8;

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
pub fn nv12_to_rgb24(src: &[u8], width: usize, height: usize) -> Vec<u8> {
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
