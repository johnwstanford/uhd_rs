/// # Safety
///
/// This function requires `ptr` to point to an allocated C string
pub unsafe fn collect_cstr(ptr:*const u8) -> String {
    let slice:&[u8] = std::slice::from_raw_parts(ptr, 1024);
    let bytes:Vec<u8> = slice.iter().take_while(|b| **b != 0).copied().collect();
    String::from_utf8(bytes).unwrap()
}

/// # Safety
///
/// This function requires `buff_len` bytes starting at `ptr` to be allocated
pub unsafe fn populate_cstr(ptr:*mut u8, buff_len:usize, data:&str) {
    let tgt_slice:&mut [u8] = std::slice::from_raw_parts_mut(ptr, buff_len);
    let src_slice:&[u8] = data.as_bytes();

    // Need one byte leftover for null termination
    assert!(src_slice.len() + 1 <= tgt_slice.len());
    let n = std::cmp::min(src_slice.len(), tgt_slice.len());
    tgt_slice[..n].copy_from_slice(&src_slice[..n]);

    tgt_slice[src_slice.len()] = 0;
}
