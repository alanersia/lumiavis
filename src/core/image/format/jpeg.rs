pub fn is_valid_jpeg(data: &[u8]) -> bool {
    if data.len() < 4 {
        return false;
    }
    // Check for JPEG SOI (Start of Image) marker
    if data[0] != 0xFF || data[1] != 0xD8 {
        return false;
    }
    // Check for JPEG EOI (End of Image) marker
    let end = data.len() - 1;
    data[end - 1] == 0xFF && data[end] == 0xD9
}
