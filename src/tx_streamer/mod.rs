
use std::ffi::CString;
use std::io::{Error, ErrorKind};

use libc::{c_char, size_t};

use crate::types::metadata::TxMetadata;

type Sample = (i16, i16);

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

impl std::io::Write for TxStreamer {

	fn write(&mut self, buffer:&[u8]) -> Result<usize, std::io::Error> { 
		let bytes_per_sample:usize = std::mem::size_of::<Sample>();
		let num_bytes:usize = buffer.len();

		if num_bytes % bytes_per_sample == 0 {

			let num_samples:usize = num_bytes / bytes_per_sample;

			let wf_u:*const u8 = &buffer[0];
			let wf_s:*const Sample = wf_u as *const _;
			let samp_buffer:&[Sample] = unsafe { std::slice::from_raw_parts(wf_s, num_samples) };

			self.send_sc16(samp_buffer).map_err(|e| Error::new(ErrorKind::Interrupted, e))

		} else {
			Err(Error::new(ErrorKind::Interrupted, "Wrong sized input for write()"))
		}
	}

	fn flush(&mut self) -> Result<(), std::io::Error> { 
		Ok(())
	}

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

	pub fn send_sc16(&mut self, buffer:&[Sample]) -> Result<usize, &'static str> {
		let mut start_idx:usize  = 0;
		let mut items_sent:usize = 0;

		while start_idx < buffer.len() {

			let num_samps:usize = std::cmp::min(self.max_num_samps, buffer.len() - start_idx);

			let start_ptr:*const (i16, i16) = &buffer[start_idx];
			let buff_ptr:*const u8 = start_ptr as *const u8;
			let mut items_sent_this_time = 0;
			let result = unsafe { 
				uhd_tx_streamer_send(self.handle, &buff_ptr, num_samps, 
					&self.tx_metadata.handle, self.timeout, &mut items_sent_this_time) 
			};

			match result {
				0 => items_sent += items_sent_this_time,
				_ => return Err("Unable to send using TxStreamer")
			}

			start_idx += self.max_num_samps;
		}

		Ok(items_sent)
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