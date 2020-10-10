
use std::ffi::CString;

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

    pub fn get_at(&self, idx:usize) -> Result<String, &'static str> {
		let buffer_init = "                                        ";
		let cstr_ans:CString = CString::new(buffer_init).map_err(|_| "Unable to create CString")?;
        match unsafe { ffi::uhd_string_vector_at(self.handle, idx, cstr_ans.as_ptr(), buffer_init.len()) } {
            0 => {
            	let ans:String = cstr_ans.into_string().map_err(|_| "Unable to convert CString to String")?;
				let ans:String = ans.trim_matches(char::from(0)).to_owned();
            	println!("{}", ans.len());

            	Ok(ans)
            },
            _ => Err("Unable to index into string vector")
        }
    }

    pub fn len(&self) -> Result<usize, &'static str> {
    	let mut ans:usize = 0;
    	match unsafe { ffi::uhd_string_vector_size(self.handle, &mut ans) } {
    		0 => Ok(ans),
    		_ => Err("Unable to retrieve string vector length")
    	}
    }
}

impl std::ops::Drop for StringVector {

	fn drop(self:&mut StringVector) {
		unsafe { ffi::uhd_string_vector_free(&mut self.handle); }
	}

}