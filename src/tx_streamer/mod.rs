
use std::ffi::CString;

use crate::ffi::usrp;
use crate::types::metadata::TxMetadata;

pub struct TxStreamer {
	handle:usize,
	num_channels:usize,
	max_num_samps:usize,	// Max number of samples per buffer per packet
	timeout:f64,
	tx_metadata:TxMetadata,
	underflow_count:usize
}

impl TxStreamer {
	
	pub fn new(num_channels:usize) -> Result<Self, &'static str> {

		if num_channels != 1 { return Err("Multiple channels in one stream aren't supported right now"); }

		let mut handle:usize = 0;
		let tx_metadata = TxMetadata::new()?;

		match unsafe { usrp::uhd_tx_streamer_make(&mut handle) } {
			0 => Ok(TxStreamer{ handle, num_channels, max_num_samps:0,
				timeout: 3.0, tx_metadata, underflow_count:0}),
			_ => Err("Unable to create TX streamer")
		}
	}

	pub fn get_handle(&self) -> usize { self.handle }

	pub fn get_max_num_samps(&mut self) -> Result<usize, &'static str> {
		match unsafe { usrp::uhd_tx_streamer_max_num_samps(self.handle, &mut self.max_num_samps) } {
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
				usrp::uhd_tx_streamer_send(self.handle, &buff_ptr, buffer.len(), 
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
		match unsafe { usrp::uhd_tx_streamer_last_error(self.handle, cstr_ans.as_ptr(), buffer_init.len()) } {
			0 => cstr_ans.into_string().map_err(|_| "Unable to convert CString to String"),
			_ => Err("Unable to get last error from TxStreamer")
		}
	}

}

impl std::ops::Drop for TxStreamer {

	fn drop(&mut self) {
		// TODO: consider checking the return value
		unsafe { crate::ffi::usrp::uhd_tx_streamer_free(&mut self.handle); }
	}

}