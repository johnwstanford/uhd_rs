
use std::ffi::CString;
use std::io::{Error, ErrorKind};

use libc::{c_char, size_t};

use crate::check_err;
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
	timeout:f64
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

			self.single_coherent_pulse(samp_buffer, None).map_err(|e| Error::new(ErrorKind::Interrupted, e))

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

		match unsafe { uhd_tx_streamer_make(&mut handle) } {
			0 => Ok(TxStreamer{ handle, max_num_samps:0, timeout: DEFAULT_TIMEOUT}),
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

	pub fn single_coherent_pulse(&mut self, buffer:&[Sample], time_spec:Option<(i64, f64)>) -> Result<usize, &'static str> {
		// The burst boundaries seem to tell UHD that phase coherence
		// isn't required in between bursts
		let md0 = TxMetadata::new(time_spec, true,  false)?;
		let md1 = TxMetadata::new(None,      false, false)?;
		let md2 = TxMetadata::new(None,      false, true )?;
		self.send_sc16(buffer, &md0, &md1, &md2)
	}

	pub fn start_coherent(&mut self, buffer:&[Sample], time_spec:Option<(i64, f64)>) -> Result<usize, &'static str> {
		let md0 = TxMetadata::new(time_spec, true,  false)?;
		let md1 = TxMetadata::new(None,      false, false)?;
		self.send_sc16(buffer, &md0, &md1, &md1)
	}

	pub fn continue_coherent(&mut self, buffer:&[Sample]) -> Result<usize, &'static str> {
		let md = TxMetadata::new(None, false, false)?;
		self.send_sc16(buffer, &md, &md, &md)
	}

	pub fn complete_coherent(&mut self, buffer:&[Sample]) -> Result<usize, &'static str> {
		let md1 = TxMetadata::new(None,      false, false)?;
		let md2 = TxMetadata::new(None,      false, true )?;
		self.send_sc16(buffer, &md1, &md1, &md2)
	}

	fn send_sc16(&mut self, buffer:&[Sample], md0:&TxMetadata, md1:&TxMetadata, md2:&TxMetadata) -> Result<usize, &'static str> {

		let mut items_sent:usize = 0;

		while items_sent < buffer.len() {

			let num_samps:usize = std::cmp::min(self.max_num_samps, buffer.len() - items_sent);
			let metadata_handle_ref:&usize = if items_sent == 0 {
				// First call
				&md0.handle
			} else if buffer.len() - items_sent > num_samps {
				// One of the calls in the middle
				&md1.handle
			} else {
				// Last call
				&md2.handle
			};

			let start_ptr:*const (i16, i16) = &buffer[items_sent];
			let buff_ptr:*const u8 = start_ptr as *const u8;
			let mut items_sent_this_time = 0;
			let result = unsafe { 
				uhd_tx_streamer_send(self.handle, &buff_ptr, num_samps, 
					metadata_handle_ref, self.timeout, &mut items_sent_this_time) 
			};

			check_err((), result)?;

			items_sent += items_sent_this_time

		}

		Ok(items_sent)
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