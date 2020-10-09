
use crate::ffi::types::metadata::{uhd_rx_metadata_error_code, uhd_rx_metadata_time_spec, RxMetadataErrorCode};
use crate::ffi::types::metadata::{uhd_tx_metadata_time_spec, uhd_tx_metadata_has_time_spec};

pub struct TxMetadata {
	pub handle:usize
}

impl TxMetadata {

	pub fn new() -> Result<TxMetadata, &'static str> {
		let mut handle:usize = 0;

		let result = unsafe { crate::ffi::types::metadata::uhd_tx_metadata_make(&mut handle, false, 0, 0.1, true, false) };

		match result {
			0 => Ok(TxMetadata{ handle }),
			_ => Err("Unable to create TxMetadata")
		}
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
		unsafe { crate::ffi::types::metadata::uhd_tx_metadata_free(&mut self.handle); }
	}

}

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

