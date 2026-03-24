pub fn is_valid_jpeg(data: &[u8]) -> bool {
    if data.len() < 4 {
        return false;
    }
    // Check for JPEG SOI (Start of Image) marker
    data[0] == 0xFF && data[1] == 0xD8
}
