
use std::ffi::CString;

use libc::{c_char, size_t};

use crate::types::metadata::TxMetadata;

#[link(name = "uhd")]
extern {
	
	// uhd_error uhd_tx_streamer_num_channels(uhd_tx_streamer_handle h, size_t *num_channels_out)
	// uhd_error uhd_tx_streamer_recv_async_msg(uhd_tx_streamer_handle h, uhd_async_metadata_handle *md, double timeout, bool *valid)

	fn uhd_tx_streamer_make(h: &mut usize) -> isize;
	fn uhd_tx_streamer_free(h: &mut usize) -> isize;
	fn uhd_tx_streamer_max_num_samps(h:usize, max_num_samps_out:&mut size_t) -> isize;
	fn uhd_tx_streamer_send(h:usize, buffs:&*const u8, samps_per_buff:size_t, md:&usize, timeout:f64, items_sent:&mut size_t) -> isize;
	fn uhd_tx_streamer_last_error(h:usize, error_out:*const c_char, strbuffer_len:size_t) -> isize;

}

pub const DEFAULT_TIMEOUT:f64 = 3.0;

pub struct TxStreamer {
	handle:usize,
	max_num_samps:usize,	// Max number of samples per buffer per packet
	timeout:f64,
	tx_metadata:TxMetadata
}

impl TxStreamer {
	
	pub fn new(num_channels:usize) -> Result<Self, &'static str> {

		if num_channels != 1 { return Err("Multiple channels in one stream aren't supported right now"); }

		let mut handle:usize = 0;
		let tx_metadata = TxMetadata::new()?;

		match unsafe { uhd_tx_streamer_make(&mut handle) } {
			0 => Ok(TxStreamer{ handle, max_num_samps:0, timeout: DEFAULT_TIMEOUT, tx_metadata}),
			_ => Err("Unable to create TX streamer")
		}
	}

	pub fn get_handle(&self) -> usize { self.handle }

	pub fn get_max_num_samps(&mut self) -> Result<usize, &'static str> {
		match unsafe { uhd_tx_streamer_max_num_samps(self.handle, &mut self.max_num_samps) } {
			0 => Ok(self.max_num_samps),
			_ => Err("Unable to get sample buffer size for the newly-created RxStream")
		}
	}

	pub fn send_sc16(&mut self, buffer:&[(i16, i16)]) -> Result<usize, &'static str> {
		if buffer.len() > self.max_num_samps {
			Err("Supplied slice is too large")
		} else {
			let buff_ptr:*const u8 = buffer.as_ptr() as *const u8;
			let mut items_sent = 0;
			let result = unsafe { 
				uhd_tx_streamer_send(self.handle, &buff_ptr, buffer.len(), 
					&self.tx_metadata.handle, self.timeout, &mut items_sent) 
			};
			match result {
				0 => Ok(items_sent),
				_ => Err("Unable to send using TxStreamer")
			}
		}
	}

	pub fn tx_metadata_time_spec(&self) -> Result<(i64, f64), &'static str> {
		self.tx_metadata.time_spec()
	}

	pub fn last_error(&self) -> Result<String, &'static str> {
		let buffer_init = "                                        ";
		let cstr_ans:CString = CString::new(buffer_init).unwrap();
		match unsafe { uhd_tx_streamer_last_error(self.handle, cstr_ans.as_ptr(), buffer_init.len()) } {
			0 => cstr_ans.into_string().map_err(|_| "Unable to convert CString to String"),
			_ => Err("Unable to get last error from TxStreamer")
		}
	}

}

impl std::ops::Drop for TxStreamer {

	fn drop(&mut self) {
		unsafe { uhd_tx_streamer_free(&mut self.handle); }
	}

}