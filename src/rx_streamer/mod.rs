
use std::ffi::CString;
use std::io::{Error, ErrorKind};

use libc::{c_char, size_t};

use crate::check_err;
use crate::types::metadata::{RxMetadata, RxMetadataErrorCode};
use crate::usrp::StreamCmd;

#[link(name = "uhd")]
extern {

	// uhd_error uhd_rx_streamer_num_channels(uhd_rx_streamer_handle h, size_t *num_channels_out)

	fn uhd_rx_streamer_make(uhd_rx_streamer_handle: &mut usize) -> isize;
	fn uhd_rx_streamer_free(uhd_rx_streamer_handle: &mut usize) -> isize;
	fn uhd_rx_streamer_max_num_samps(h:usize, max_num_samps_out:&mut size_t) -> isize;
	fn uhd_rx_streamer_recv(h:usize, buffs:&*const u8, samps_per_buff:size_t, md:&usize, timeout:f64, one_packet:bool, items_recvd:&mut size_t) -> isize;
	fn uhd_rx_streamer_issue_stream_cmd(h:usize, stream_cmd:&StreamCmd) -> isize;
	fn uhd_rx_streamer_last_error(h:usize, error_out:*const c_char, strbuffer_len:size_t) -> isize;
}

pub struct RxStreamer {
	pub timeout:f64,
	handle:usize,
	max_num_samps:usize,	// Max number of samples per buffer per packet
	rx_metadata:RxMetadata,
	overflow_count:usize
}

impl std::io::Read for RxStreamer {

	fn read(&mut self, buff: &mut [u8]) -> std::result::Result<usize, std::io::Error> { 

		let mut items_recvd = 0;

		let result = unsafe { 
			uhd_rx_streamer_recv(self.handle, &(&(buff[0]) as *const u8), 
				std::cmp::min(self.max_num_samps, buff.len()), 
				&self.rx_metadata.handle, self.timeout, false, &mut items_recvd)
		};

		if result != 0 { return Err(Error::new(ErrorKind::Interrupted, "Unable to receive from RX stream")); }

		Ok(items_recvd*4)
	}

}

impl RxStreamer {
	
	pub fn new(num_channels:usize) -> Result<Self, &'static str> {

		if num_channels != 1 { return Err("Multiple channels in one stream aren't supported right now"); }

		let mut handle:usize = 0;
		let rx_metadata = RxMetadata::new()?;

		match unsafe { uhd_rx_streamer_make(&mut handle) } {
			0 => Ok(RxStreamer{ handle, max_num_samps:0,
				timeout: 1.0, rx_metadata, overflow_count:0}),
			_ => Err("Unable to create RX streamer")
		}
	}

	pub fn get_handle(&self) -> usize { self.handle }

	pub fn read_sc16(&mut self, buff: &mut [(i16, i16)], timeout:Option<f64>) -> Result<(usize, (i64, f64)), &'static str> { 
		// If you're migrating code that used this function before `timeout` was added, then using `None` for this
		// parameter will give the same behavior as before

		let start_time = std::time::Instant::now();

		let mut current_idx = 0;
		let mut items_recvd = 0;

		let mut time_spec = (0, 0.0);

		while current_idx < buff.len() {
			let result = unsafe { 
				uhd_rx_streamer_recv(self.handle, 
					&(&(buff[current_idx]) as *const (i16,i16) as *const u8), 		// This is a pointer to a pointer
					std::cmp::min(self.max_num_samps, buff.len() - current_idx),	// Max number of samples to send (samples, not bytes) 
					&self.rx_metadata.handle, 	// Pointer to metadata in which to receive results
					self.timeout, 				// Timeout in seconds
					false, 						// Whether or not to send a single packet; TODO: look into the effect of this
					&mut items_recvd)			// Output variable for number of samples received
			};

			check_err((), result)?;

			if current_idx == 0 {
				// This is the first call, so this is the time spec we want to save.  We want the return value of the entire function
				// call to be the timestamp of the first sample of the entire buffer, not a timestamp somewhere in the middle
				time_spec = self.rx_metadata.time_spec()?;
			}

			current_idx += items_recvd;

			// If timeout is None, then there's no timeout and this function just blocks
			// until it fills the buffer, no matter how long that takes.
			if let Some(dt) = &timeout {
				if start_time.elapsed().as_secs_f64() > *dt {
					return Ok((current_idx, time_spec));
				}
			}

		}

		Ok((current_idx, time_spec))
	}

	// Simple API calls
	pub fn get_max_num_samps(&mut self) -> Result<usize, &'static str> {
		match unsafe { uhd_rx_streamer_max_num_samps(self.handle, &mut self.max_num_samps) } {
			0 => Ok(self.max_num_samps),
			_ => Err("Unable to get sample buffer size for the newly-created RxStream")
		}
	}

	pub fn stream(&mut self, stream_cmd:&StreamCmd) -> Result<(), &'static str> {
		match unsafe { uhd_rx_streamer_issue_stream_cmd(self.handle, stream_cmd) } {
			0 => Ok(()),
			_ => Err("Unable to issue stream command")
		}
	}

	pub fn rx_metadata_ok(&mut self) -> Result<(), &'static str> {
		match self.rx_metadata.error_code()? {
			RxMetadataErrorCode::None => Ok(()),
			RxMetadataErrorCode::Overflow => { self.overflow_count += 1; Ok(()) },
			ec => {
				eprintln!("ERR: {:?}", ec);
				Err("RxMetadata error code other than None or Overflow")
			}
		}
	}

	pub fn last_error(&self) -> Result<String, &'static str> {
		let buffer_init = "                                        ";
		let cstr_ans:CString = CString::new(buffer_init).unwrap();
		match unsafe { uhd_rx_streamer_last_error(self.handle, cstr_ans.as_ptr(), buffer_init.len()) } {
			0 => cstr_ans.into_string().map_err(|_| "Unable to convert CString to String"),
			_ => Err("Unable to get last error from RxStreamer")
		}
	}

}

impl std::ops::Drop for RxStreamer {

	fn drop(&mut self) {
		// Issue a stop command before dropping
		let stream_cmd_stop  = StreamCmd::stop_continuous_now();
		match self.stream(&stream_cmd_stop) {
			Ok(_) => (),
			Err(_) => eprintln!("WARN: Error when calling stop_continuous_now in RxStreamer::drop")
		}

		unsafe { uhd_rx_streamer_free(&mut self.handle); }
	}

}