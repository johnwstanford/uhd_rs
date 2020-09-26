
use std::ffi::CString;

use crate::ffi::usrp;
use crate::ffi::types::metadata::RxMetadataErrorCode;
use crate::types::metadata::RxMetadata;

const BUFFER_SIZE:usize = 4096*8;

pub struct RxStreamer {
	handle:usize,
	num_channels:usize,
	max_num_samps:usize,	// Max number of samples per buffer per packet
	pub buffer:[u8; BUFFER_SIZE],	// TODO: consider putting this on the heap
	timeout:f64,
	rx_metadata:RxMetadata,
	overflow_count:usize
}

impl RxStreamer {
	
	pub fn new(num_channels:usize) -> Result<Self, &'static str> {

		if num_channels != 1 { return Err("Multiple channels in one stream aren't supported right now"); }

		let mut handle:usize = 0;
		let rx_metadata = RxMetadata::new()?;
		let buffer = [0u8; BUFFER_SIZE];

		match unsafe { usrp::uhd_rx_streamer_make(&mut handle) } {
			0 => Ok(RxStreamer{ handle, num_channels, max_num_samps:0, buffer,
				timeout: 3.0, rx_metadata, overflow_count:0}),
			_ => Err("Unable to create RX streamer")
		}
	}

	pub fn get_handle(&self) -> usize { self.handle }

	pub fn get_max_num_samps(&mut self) -> Result<usize, &'static str> {
		match unsafe { usrp::uhd_rx_streamer_max_num_samps(self.handle, &mut self.max_num_samps) } {
			0 => match self.max_num_samps {
				n if n > self.buffer.len() => Err("Statically-sized buffer is too small"),
				n => Ok(n)
			},
			_ => Err("Unable to get sample buffer size for the newly-created RxStream")
		}
	}

	pub fn stream(&mut self, stream_cmd:&usrp::StreamCmd) -> Result<(), &'static str> {
		match unsafe { usrp::uhd_rx_streamer_issue_stream_cmd(self.handle, stream_cmd) } {
			0 => Ok(()),
			_ => Err("Unable to issue stream command")
		}
	}

	pub fn recv(&mut self, one_packet:bool) -> Result<usize, &'static str> {
		let buff_ptr:*const u8 = self.buffer.as_ptr() as *const u8;
		let mut items_recvd = 0;
		let result = unsafe { 
			usrp::uhd_rx_streamer_recv(self.handle, &buff_ptr, self.max_num_samps, 
				&self.rx_metadata.handle, self.timeout, one_packet, &mut items_recvd) 
		};
		match result {
			0 => Ok(items_recvd),
			_ => Err("Unable to receive from RxStreamer")
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

	pub fn rx_metadata_time_spec(&self) -> Result<(i64, f64), &'static str> {
		self.rx_metadata.time_spec()
	}

	pub fn last_error(&self) -> Result<String, &'static str> {
		let buffer_init = "                                        ";
		let cstr_ans:CString = CString::new(buffer_init).unwrap();
		match unsafe { usrp::uhd_rx_streamer_last_error(self.handle, cstr_ans.as_ptr(), buffer_init.len()) } {
			0 => cstr_ans.into_string().map_err(|_| "Unable to convert CString to String"),
			_ => Err("Unable to get last error from RxStreamer")
		}
	}

}

impl std::ops::Drop for RxStreamer {

	fn drop(&mut self) {
		// TODO: consider checking the return value
		unsafe { crate::ffi::usrp::uhd_rx_streamer_free(&mut self.handle); }
	}

}