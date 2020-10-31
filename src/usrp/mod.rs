
use std::ffi::CString;

use libc::{size_t, c_char};

use crate::rx_streamer::RxStreamer;

#[link(name = "uhd")]
extern {

	fn uhd_usrp_get_num_mboards(h:usize, num_mboards_out:&mut size_t) -> isize;
	// uhd_error uhd_usrp_set_time_source(uhd_usrp_handle h, const char* time_source, size_t mboard)
	fn uhd_usrp_get_time_source(h:usize, mboard:size_t, time_source_out:*const c_char, strbuffer_len:size_t) -> isize;
	// uhd_error uhd_usrp_get_time_sources(uhd_usrp_handle h, size_t mboard, uhd_string_vector_handle *time_sources_out)
	// uhd_error uhd_usrp_set_clock_source(uhd_usrp_handle h, const char* clock_source, size_t mboard)
	// uhd_error uhd_usrp_get_clock_source(uhd_usrp_handle h, size_t mboard, char* clock_source_out, size_t strbuffer_len)
	// uhd_error uhd_usrp_get_clock_sources(uhd_usrp_handle h, size_t mboard, uhd_string_vector_handle *clock_sources_out)
	// uhd_error uhd_usrp_set_clock_source_out(uhd_usrp_handle h, bool enb, size_t mboard)
	// uhd_error uhd_usrp_set_time_source_out(uhd_usrp_handle h, bool enb, size_t mboard)

	fn uhd_usrp_free(uhd_usrp_handle: &mut usize);	
}

pub struct USRP {
	handle:usize,
	last_commanded_rate:Option<f64>,
	last_commanded_gain:Option<f64>,
	last_commanded_bw:Option<f64>,
	opt_rx_streamer:Option<RxStreamer>,
}

mod impl_static;
mod impl_rx;
mod impl_tx;

impl USRP {

	pub fn num_mboards(&self) -> Result<usize, &'static str> {
		let mut ans = 0;
		match unsafe { uhd_usrp_get_num_mboards(self.handle, &mut ans) } {
			0 => Ok(ans),
			_ => Err("Unable to get the number of motherboards"),
		}
	}

	pub fn get_time_source(&self, mboard:usize) -> Result<String, &'static str> {
		let buffer_init = "                                        ";
		let cstr_ans:CString = CString::new(buffer_init).map_err(|_| "Unable to create CString")?;
        match unsafe { uhd_usrp_get_time_source(self.handle, mboard, cstr_ans.as_ptr(), buffer_init.len()) } {
            0 => {
            	let ans:String = cstr_ans.into_string().map_err(|_| "Unable to convert CString to String")?;
				let ans:String = ans.trim_matches(char::from(0)).to_owned();
            	Ok(ans)
            },
            _ => Err("Unable to index into string vector")
        }

	}

}

impl std::ops::Drop for USRP {

	fn drop(&mut self) { 
		// TODO: consider checking the return value; right now we're not
		unsafe { uhd_usrp_free(&mut self.handle); } 
	}

}