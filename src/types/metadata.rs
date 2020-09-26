
use crate::ffi::types::metadata::{uhd_rx_metadata_error_code, uhd_rx_metadata_time_spec, RxMetadataErrorCode};

pub struct RxMetadata {
	pub handle:usize
}

impl RxMetadata {

	pub fn new() -> Result<RxMetadata, &'static str> {
		let mut handle:usize = 0;

		let result = unsafe { crate::ffi::types::metadata::uhd_rx_metadata_make(&mut handle) };

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
		unsafe { crate::ffi::types::metadata::uhd_rx_metadata_free(&mut self.handle); }
	}

}