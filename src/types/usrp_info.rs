
use std::ffi::CStr;

use libc::c_char;

#[repr(C)]
// RxInfo and TxInfo have the exact same structure,
// so I'm just going to use the same struct
pub struct Info {
    mboard_id:*const c_char,		// Motherboard ID
    mboard_name:*const c_char,		// Motherboard name
    mboard_serial:*const c_char,	// Motherboard serial
    id:*const c_char,				// Daughterboard ID
    subdev_name:*const c_char,		// Subdev name
   	subdev_spec:*const c_char, 		// Subdev spec
    serial:*const c_char,			// Daughterboard serial
    antenna:*const c_char			// Daughterboard antenna
}

impl Info {

	pub fn null() -> Self {
		Self { 
		    mboard_id:(0 as *const c_char),
		    mboard_name:(0 as *const c_char),
		    mboard_serial:(0 as *const c_char),
		    id:(0 as *const c_char),
		    subdev_name:(0 as *const c_char),
		   	subdev_spec:(0 as *const c_char),
		    serial:(0 as *const c_char),
		    antenna:(0 as *const c_char)
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
		if self.mboard_name == 0 as *const c_char {
			Err("Tried to retrieve mboard_name before the memory was initialized")
		} else {
			unsafe { Ok(CStr::from_ptr(self.mboard_name).to_str().map_err(|_| "Bad UTF-8")?.to_owned()) }
		}
	}

	pub fn mboard_serial(&self) -> Result<String, &'static str> {
		if self.mboard_serial == 0 as *const c_char {
			Err("Tried to retrieve mboard_serial before the memory was initialized")
		} else {
			unsafe { Ok(CStr::from_ptr(self.mboard_serial).to_str().map_err(|_| "Bad UTF-8")?.to_owned()) }
		}
	}

	pub fn id(&self) -> Result<String, &'static str> {
		if self.id == 0 as *const c_char {
			Err("Tried to retrieve id before the memory was initialized")
		} else {
			unsafe { Ok(CStr::from_ptr(self.id).to_str().map_err(|_| "Bad UTF-8")?.to_owned()) }
		}
	}

	pub fn subdev_name(&self) -> Result<String, &'static str> {
		if self.subdev_name == 0 as *const c_char {
			Err("Tried to retrieve subdev_name before the memory was initialized")
		} else {
			unsafe { Ok(CStr::from_ptr(self.subdev_name).to_str().map_err(|_| "Bad UTF-8")?.to_owned()) }
		}
	}

	pub fn subdev_spec(&self) -> Result<String, &'static str> {
		if self.subdev_spec == 0 as *const c_char {
			Err("Tried to retrieve subdev_spec before the memory was initialized")
		} else {
			unsafe { Ok(CStr::from_ptr(self.subdev_spec).to_str().map_err(|_| "Bad UTF-8")?.to_owned()) }
		}
	}

	pub fn serial(&self) -> Result<String, &'static str> {
		if self.serial == 0 as *const c_char {
			Err("Tried to retrieve serial before the memory was initialized")
		} else {
			unsafe { Ok(CStr::from_ptr(self.serial).to_str().map_err(|_| "Bad UTF-8")?.to_owned()) }
		}
	}

	pub fn antenna(&self) -> Result<String, &'static str> {
		if self.antenna == 0 as *const c_char {
			Err("Tried to retrieve antenna before the memory was initialized")
		} else {
			unsafe { Ok(CStr::from_ptr(self.antenna).to_str().map_err(|_| "Bad UTF-8")?.to_owned()) }
		}
	}

}

impl std::ops::Drop for Info {

	fn drop(&mut self) { unsafe { uhd_usrp_rx_info_free(self); }}

}

#[link(name = "uhd")]
extern {
	pub fn uhd_usrp_rx_info_free(rx_info:&mut Info) -> isize;
	pub fn uhd_usrp_tx_info_free(tx_info:&mut Info) -> isize;
}