use std::ffi::CString;
use libc::c_char;
use crate::c_interop::collect_cstr;

#[link(name = "uhd")]
extern {

    fn uhd_subdev_spec_make(h: *mut usize, markup: *const c_char) -> isize;
    fn uhd_subdev_spec_free(h: *mut usize) -> isize;

    fn uhd_subdev_spec_size(h: usize, size_out: *mut isize) -> isize;
    fn uhd_subdev_spec_push_back(h: usize, markup: *const c_char) -> isize;

    /*
    UHD_API uhd_error uhd_subdev_spec_at(uhd_subdev_spec_handle h, size_t num, uhd_subdev_spec_pair_t* subdev_spec_pair_out);
    UHD_API uhd_error uhd_subdev_spec_to_pp_string(uhd_subdev_spec_handle h, char* pp_string_out, size_t strbuffer_len);
     */

    fn uhd_subdev_spec_to_string(h: usize, string_out: *mut u8, strbuffer_len: usize) -> isize;
    fn uhd_subdev_spec_last_error(h: usize, error_out: *mut u8, strbuffer_len: usize) -> isize;
}

#[cfg(test)]
pub mod tests;

pub struct SubdevSpec {
    pub handle: usize,
}

impl SubdevSpec {

    pub fn new(markup: &str) -> Result<Self, &'static str> {
        let mut handle: usize = 0;
        let markup_c = CString::new(markup).map_err(|_| "Unable to build CString from subdev spec markup")?;
        unsafe {

            match uhd_subdev_spec_make(&mut handle, markup_c.as_ptr()) {
                0 => Ok(Self{handle}),
                x => {
                    eprintln!("Return value {}", x);
                    Err("Nonzero return value from uhd_subdev_spec_make")
                }
            }
        }
    }

    pub fn last_error(&mut self) -> Result<String, &'static str> {
        unsafe {
            let mut err: Vec<u8> = vec![0x00; 128];
            match uhd_subdev_spec_last_error(self.handle, err.as_mut_ptr(), err.len()) {
                0 => Ok(collect_cstr(err.as_ptr())),
                _ => Err("Failed to get last error"),
            }

        }
    }

    pub fn to_string(&mut self) -> Result<String, &'static str> {
        unsafe {
            let mut err: Vec<u8> = vec![0x00; 128];
            match uhd_subdev_spec_to_string(self.handle, err.as_mut_ptr(), err.len()) {
                0 => Ok(collect_cstr(err.as_ptr())),
                _ => Err("Failed to get subdev spec as a string"),
            }

        }
    }

    pub fn len(&mut self) -> Result<usize, &'static str> {
        let mut ans: isize = 0;
        unsafe {
            match uhd_subdev_spec_size(self.handle, &mut ans) {
                0 => Ok(ans as usize),
                _ => Err("Nonzero return value in SubdevSpec::len"),
            }
        }
    }

    pub fn push_back(&mut self, markup: &str) -> Result<(), &'static str> {
        let markup_c = CString::new(markup).map_err(|_| "Unable to build CString from subdev spec markup")?;
        unsafe {
            match uhd_subdev_spec_push_back(self.handle, markup_c.as_ptr()) {
                0 => Ok(()),
                _ => Err("Nonzero return value in SubdevSpec::push_back"),
            }
        }
    }

}

impl Drop for SubdevSpec {
    fn drop(&mut self) {
        unsafe {
            if uhd_subdev_spec_free(&mut self.handle) != 0 {
                eprintln!("WARN: Nonzero return value in SubdevSpec::drop")
            }
        }
    }
}