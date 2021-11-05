use std::path::Path;

pub fn write_sc16_to_file<P: AsRef<Path>>(path:P, data:&[(i16, i16)]) -> Result<(), &'static str> {

    let data_u8: &[u8] = unsafe {
        let data_ptr: *const u8 = data.as_ptr() as *const u8;
        std::slice::from_raw_parts(data_ptr, data.len() * std::mem::size_of::<(i16, i16)>())
    };

    std::fs::write(path, data_u8).map_err(|_| "Unable to write &[(i16, i16)] to a file")

}