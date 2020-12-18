
use crate::check_err;

#[link(name = "uhd")]
extern {

	// metadata.h:94
	// uhd_error uhd_rx_metadata_make(uhd_rx_metadata_handle* handle)
	pub fn uhd_rx_metadata_make(handle: &mut usize) -> isize;

	// uhd_error uhd_rx_metadata_free(uhd_rx_metadata_handle* handle)
	pub fn uhd_rx_metadata_free(handle: &mut usize) -> isize;

	// uhd_error uhd_rx_metadata_has_time_spec(uhd_rx_metadata_handle h, bool *result_out)

	// uhd_error uhd_rx_metadata_time_spec(uhd_rx_metadata_handle h, int64_t *full_secs_out, double *frac_secs_out)
	pub fn uhd_rx_metadata_time_spec(h:usize, full_secs_out:&mut i64, frac_secs_out:&mut f64) -> isize;

	// uhd_error uhd_rx_metadata_more_fragments(uhd_rx_metadata_handle h, bool *result_out)
	// uhd_error uhd_rx_metadata_fragment_offset(uhd_rx_metadata_handle h, size_t *fragment_offset_out)
	// uhd_error uhd_rx_metadata_start_of_burst(uhd_rx_metadata_handle h, bool *result_out)
	// uhd_error uhd_rx_metadata_end_of_burst(uhd_rx_metadata_handle h, bool *result_out)
	// uhd_error uhd_rx_metadata_out_of_sequence(uhd_rx_metadata_handle h, bool *result_out)
	// uhd_error uhd_rx_metadata_to_pp_string(uhd_rx_metadata_handle h, char* pp_string_out, size_t strbuffer_len)

	// uhd_error uhd_rx_metadata_error_code(uhd_rx_metadata_handle h, uhd_rx_metadata_error_code_t *error_code_out)
	pub fn uhd_rx_metadata_error_code(h:usize, error_code_out:&mut RxMetadataErrorCode) -> isize;
	
	// uhd_error uhd_rx_metadata_strerror(uhd_rx_metadata_handle h, char* strerror_out, size_t strbuffer_len)
	// uhd_error uhd_rx_metadata_last_error(uhd_rx_metadata_handle h, char* error_out, size_t strbuffer_len)
	
	// uhd_error uhd_tx_metadata_make(uhd_tx_metadata_handle* handle, bool has_time_spec, int64_t full_secs, double frac_secs, bool start_of_burst, bool end_of_burst)
	pub fn uhd_tx_metadata_make(handle:&mut usize, has_time_spec:bool, full_secs:i64, frac_secs:f64, start_of_burst:bool, end_of_burst:bool) -> isize;
	
	// uhd_error uhd_tx_metadata_free(uhd_tx_metadata_handle* handle)
	pub fn uhd_tx_metadata_free(handle:&mut usize) -> isize;

	// uhd_error uhd_tx_metadata_has_time_spec(uhd_tx_metadata_handle h, bool *result_out)
	pub fn uhd_tx_metadata_has_time_spec(h:usize, result_out:&mut bool) -> isize;
	
	// uhd_error uhd_tx_metadata_time_spec(uhd_tx_metadata_handle h, int64_t *full_secs_out, double *frac_secs_out)
	pub fn uhd_tx_metadata_time_spec(h:usize, full_secs_out:&mut i64, frac_secs_out:&mut f64) -> isize;

	// uhd_error uhd_tx_metadata_start_of_burst(uhd_tx_metadata_handle h, bool *result_out)
	// uhd_error uhd_tx_metadata_end_of_burst(uhd_tx_metadata_handle h, bool *result_out)
	// uhd_error uhd_tx_metadata_last_error(uhd_tx_metadata_handle h, char* error_out, size_t strbuffer_len)
	// uhd_error uhd_async_metadata_make(uhd_async_metadata_handle* handle)
	// uhd_error uhd_async_metadata_free(uhd_async_metadata_handle* handle)
	// uhd_async_metadata_channel(uhd_async_metadata_handle h,size_t *channel_out)
	// uhd_error uhd_async_metadata_has_time_spec(uhd_async_metadata_handle h, bool *result_out)
	// uhd_error uhd_async_metadata_time_spec(uhd_async_metadata_handle h, int64_t *full_secs_out, double *frac_secs_out)
	// uhd_error uhd_async_metadata_event_code(uhd_async_metadata_handle h, uhd_async_metadata_event_code_t *event_code_out)
	// uhd_error uhd_async_metadata_user_payload(uhd_async_metadata_handle h, uint32_t user_payload_out[4])
	// uhd_error uhd_async_metadata_last_error(uhd_async_metadata_handle h, char* error_out, size_t strbuffer_len)

}

#[repr(C)]
#[derive(Debug)]
pub enum RxMetadataErrorCode {
    None 		= 0x0,		// No error code associated with this metadata
    Timeout    	= 0x1,		// No packet received, implementation timed out
    LateCommand = 0x2,		// A stream command was issued in the past
    BrokenChain = 0x4,		// Expected another stream command
    Overflow    = 0x8,		// Overflow or sequence error
    Alignment   = 0xC,		// Multi-channel alignment failed
    BadPacket   = 0xF		// The packet could not be parsed
}

pub struct TxMetadata {
	pub handle:usize
}

impl TxMetadata {

	pub fn new(time_spec:Option<(i64, f64)>, start_of_burst:bool, end_of_burst:bool) -> Result<TxMetadata, &'static str> {
		let mut handle:usize = 0;

		let (has_time_spec, full_secs, frac_secs) = match time_spec {
			Some((full, frac)) => (true, full, frac),
			None               => (false, 0, 0.0),
		};

		let result = unsafe { uhd_tx_metadata_make(&mut handle, has_time_spec, full_secs, frac_secs, start_of_burst, end_of_burst) };
		check_err(TxMetadata{ handle }, result)
	}

	pub fn has_time_spec(&self) -> Result<bool, &'static str> {
		let mut ans:bool = false;
		match unsafe { uhd_tx_metadata_has_time_spec(self.handle, &mut ans) } {
			0 => Ok(ans),
			_ => Err("Unable to determine whether time spec is available")
		}
	}

	pub fn time_spec(&self) -> Result<(i64, f64), &'static str> {
		let mut full_secs:i64 = 0;
		let mut frac_secs:f64 = 0.0;
		match unsafe { uhd_tx_metadata_time_spec(self.handle, &mut full_secs, &mut frac_secs) } {
			0 => Ok((full_secs, frac_secs)),
			_ => Err("Unable to retrieve time of the first sample")
		}
	}

}

impl std::ops::Drop for TxMetadata {

	fn drop(&mut self) {
		unsafe { uhd_tx_metadata_free(&mut self.handle); }
	}

}

pub struct RxMetadata {
	pub handle:usize
}

impl RxMetadata {

	pub fn new() -> Result<RxMetadata, &'static str> {
		let mut handle:usize = 0;

		let result = unsafe { uhd_rx_metadata_make(&mut handle) };

		match result {
			0 => Ok(RxMetadata{ handle }),
			_ => Err("Unable to create RxMetadata")
		}
	}

	pub fn error_code(&self) -> Result<RxMetadataErrorCode, &'static str> {
		let mut ec = RxMetadataErrorCode::None;
		match unsafe { uhd_rx_metadata_error_code(self.handle, &mut ec) } {
			0 => Ok(ec),
			_ => Err("Request for error code failed")
		}
	}

	pub fn time_spec(&self) -> Result<(i64, f64), &'static str> {
		let mut full_secs:i64 = 0;
		let mut frac_secs:f64 = 0.0;
		match unsafe { uhd_rx_metadata_time_spec(self.handle, &mut full_secs, &mut frac_secs) } {
			0 => Ok((full_secs, frac_secs)),
			_ => Err("Unable to retrieve time of the first sample")
		}
	}

}

impl std::ops::Drop for RxMetadata {

	fn drop(&mut self) {
		unsafe { uhd_rx_metadata_free(&mut self.handle); }
	}

}

