
use libc::{c_char, size_t};

use std::ffi::CString;

#[link(name = "uhd")]
extern {

	pub fn uhd_string_vector_make(h:&mut usize) -> isize;
	pub fn uhd_string_vector_free(h:&mut usize) -> isize;

	// uhd_error uhd_string_vector_push_back(uhd_string_vector_handle *h, const char* value)
	
	pub fn uhd_string_vector_at(h:usize, index:size_t, value_out:*const c_char, strbuffer_len:size_t) -> isize;
	pub fn uhd_string_vector_size(h:usize, size_out:&mut usize) -> isize;

	// uhd_error uhd_string_vector_last_error(uhd_string_vector_handle h, char* error_out, size_t strbuffer_len)

}

pub struct StringVector {
	pub handle:usize
}

impl StringVector {

	pub fn new() -> Result<Self, &'static str> {
		let mut handle:usize = 0;
		match unsafe { uhd_string_vector_make(&mut handle) } {
			0 => Ok(Self{ handle }),
			_ => Err("Unable to create string_vector")
		}
	}

	pub fn get_rust_vec(&self) -> Result<Vec<String>, &'static str> {
		let mut ans:Vec<String> = vec![];
		for idx in 0..(self.len()?) {
			ans.push(self.get_at(idx)?);
		}
		Ok(ans)		
	}

    pub fn get_at(&self, idx:usize) -> Result<String, &'static str> {
		let buffer_init = "                                        ";
		let cstr_ans:CString = CString::new(buffer_init).map_err(|_| "Unable to create CString")?;
        match unsafe { uhd_string_vector_at(self.handle, idx, cstr_ans.as_ptr(), buffer_init.len()) } {
            0 => {
            	let ans:String = cstr_ans.into_string().map_err(|_| "Unable to convert CString to String")?;
				let ans:String = ans.trim_matches(char::from(0)).to_owned();
            	Ok(ans)
            },
            _ => Err("Unable to index into string vector")
        }
    }

    pub fn len(&self) -> Result<usize, &'static str> {
    	let mut ans:usize = 0;
    	match unsafe { uhd_string_vector_size(self.handle, &mut ans) } {
    		0 => Ok(ans),
    		_ => Err("Unable to retrieve string vector length")
    	}
    }
}

impl std::ops::Drop for StringVector {

	fn drop(self:&mut StringVector) {
		unsafe { uhd_string_vector_free(&mut self.handle); }
	}

}