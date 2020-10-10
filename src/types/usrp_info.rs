
use std::ffi::CStr;

use libc::c_char;

#[repr(C)]
pub struct RxInfo {
    mboard_id:*const c_char,		// Motherboard ID
    mboard_name:*const c_char,		// Motherboard name
    mboard_serial:*const c_char,	// Motherboard serial
    rx_id:*const c_char,			// RX daughterboard ID
    rx_subdev_name:*const c_char,	// RX subdev name
   	rx_subdev_spec:*const c_char, 	// RX subdev spec
    rx_serial:*const c_char,		// RX daughterboard serial
    rx_antenna:*const c_char		// RX daughterboard antenna
}

impl RxInfo {

	pub fn null() -> Self {
		Self { 
		    mboard_id:(0 as *const c_char),
		    mboard_name:(0 as *const c_char),
		    mboard_serial:(0 as *const c_char),
		    rx_id:(0 as *const c_char),
		    rx_subdev_name:(0 as *const c_char),
		   	rx_subdev_spec:(0 as *const c_char),
		    rx_serial:(0 as *const c_char),
		    rx_antenna:(0 as *const c_char)
		}
	}

	pub fn mboard_id(&self) -> Result<String, &'static str> {
		if self.mboard_id == 0 as *const c_char {
			Err("Tried to retrieve mboard_id before the memory was initialized")
		} else {
			unsafe { Ok(CStr::from_ptr(self.mboard_id).to_str().map_err(|_| "Bad UTF-8")?.to_owned()) }
		}
	}

	pub fn mboard_name(&self) -> Result<String, &'static str> {
		if self.rx_antenna == 0 as *const c_char {
			Err("Tried to retrieve mboard_name before the memory was initialized")
		} else {
			unsafe { Ok(CStr::from_ptr(self.mboard_name).to_str().map_err(|_| "Bad UTF-8")?.to_owned()) }
		}
	}

	pub fn mboard_serial(&self) -> Result<String, &'static str> {
		if self.rx_antenna == 0 as *const c_char {
			Err("Tried to retrieve mboard_serial before the memory was initialized")
		} else {
			unsafe { Ok(CStr::from_ptr(self.mboard_serial).to_str().map_err(|_| "Bad UTF-8")?.to_owned()) }
		}
	}

	pub fn rx_id(&self) -> Result<String, &'static str> {
		if self.rx_antenna == 0 as *const c_char {
			Err("Tried to retrieve rx_id before the memory was initialized")
		} else {
			unsafe { Ok(CStr::from_ptr(self.rx_id).to_str().map_err(|_| "Bad UTF-8")?.to_owned()) }
		}
	}

	pub fn rx_subdev_name(&self) -> Result<String, &'static str> {
		if self.rx_antenna == 0 as *const c_char {
			Err("Tried to retrieve rx_subdev_name before the memory was initialized")
		} else {
			unsafe { Ok(CStr::from_ptr(self.rx_subdev_name).to_str().map_err(|_| "Bad UTF-8")?.to_owned()) }
		}
	}

	pub fn rx_subdev_spec(&self) -> Result<String, &'static str> {
		if self.rx_antenna == 0 as *const c_char {
			Err("Tried to retrieve rx_subdev_spec before the memory was initialized")
		} else {
			unsafe { Ok(CStr::from_ptr(self.rx_subdev_spec).to_str().map_err(|_| "Bad UTF-8")?.to_owned()) }
		}
	}

	pub fn rx_serial(&self) -> Result<String, &'static str> {
		if self.rx_antenna == 0 as *const c_char {
			Err("Tried to retrieve rx_serial before the memory was initialized")
		} else {
			unsafe { Ok(CStr::from_ptr(self.rx_serial).to_str().map_err(|_| "Bad UTF-8")?.to_owned()) }
		}
	}

	pub fn rx_antenna(&self) -> Result<String, &'static str> {
		if self.rx_antenna == 0 as *const c_char {
			Err("Tried to retrieve rx_antenna before the memory was initialized")
		} else {
			unsafe { Ok(CStr::from_ptr(self.rx_antenna).to_str().map_err(|_| "Bad UTF-8")?.to_owned()) }
		}
	}

}

impl std::ops::Drop for RxInfo {

	fn drop(&mut self) { unsafe { uhd_usrp_rx_info_free(self); }}

}

#[link(name = "uhd")]
extern {
	// uhd_error uhd_usrp_rx_info_free(uhd_usrp_rx_info_t *rx_info)
	pub fn uhd_usrp_rx_info_free(rx_info:&mut RxInfo) -> isize;
	
	// uhd_error uhd_usrp_tx_info_free(uhd_usrp_tx_info_t *tx_info)
}