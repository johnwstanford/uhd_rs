
use crate::ffi::types::{string_vector as ffi};

pub struct StringVector {
	pub handle:usize
}

impl StringVector {

	pub fn new() -> Result<Self, &'static str> {
		let mut handle:usize = 0;
		match unsafe { ffi::uhd_string_vector_make(&mut handle) } {
			0 => Ok(Self{ handle }),
			_ => Err("Unable to create string_vector")
		}
	}

}

impl std::ops::Drop for StringVector {

	fn drop(self:&mut StringVector) {
		unsafe { ffi::uhd_string_vector_free(&mut self.handle); }
	}

}