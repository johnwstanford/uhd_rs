
use std::ffi::CString;
use libc::c_char;

use crate::types::string_vector::StringVector;

#[link(name = "uhd")]
extern {
	fn uhd_usrp_find(args:*const c_char, strings_out:&mut usize) -> isize;
	fn uhd_usrp_make(uhd_usrp_handle: &mut usize, args: *const c_char) -> isize;	
}

impl super::USRP {

	pub fn find(args:&str) -> Result<Vec<String>, &'static str> {

		let args = CString::new(args).map_err(|_| "Unable to create CString; check for null characters")?;
		let mut string_vec = StringVector::new()?;
		match unsafe { uhd_usrp_find(args.as_ptr(), &mut string_vec.handle) } {
			0 => Ok(string_vec.get_rust_vec()?),
			_ => Err("Unable to find USRP devices")
		}

	}

	pub fn new(args:&str) -> Result<Self, &'static str> {

		let args = CString::new(args).map_err(|_| "Unable to create CString; check for null characters")?;

		let mut handle:usize = 0;

		let result = unsafe { uhd_usrp_make(&mut handle, args.as_ptr()) };

		match result {
			0 => Ok(Self{ handle }),
			_ => Err("Unable to create USRP")
		}

	}


}

