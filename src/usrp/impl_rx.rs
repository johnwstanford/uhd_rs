
use libc::{c_char, size_t};

use crate::types::usrp_info::Info;

#[link(name = "uhd")]
extern {

	fn uhd_usrp_get_rx_info(h:usize, chan:size_t, info_out:&mut Info) -> isize;
	fn uhd_usrp_set_rx_rate(h:usize, rate:f64, chan:size_t) -> isize;
	fn uhd_usrp_get_rx_rate(h:usize, chan:size_t, rate_out:&mut f64) -> isize;

}

impl super::USRP {

	pub fn get_rx_info(&self, chan:usize) -> Result<Info, &'static str> {
		let mut ans = Info::null();
		match unsafe { uhd_usrp_get_rx_info(self.handle, chan, &mut ans) } {
			0 => Ok(ans),
			_ => Err("Unable to get RX info")
		}
	}

	pub fn set_rx_rate(&mut self, rate:f64, chan:usize) -> Result<(), &'static str> {
		match unsafe { uhd_usrp_set_rx_rate(self.handle, rate, chan) } {
			0 => Ok(()),
			_ => Err("Unable to set RX rate")
		}
	}

	pub fn get_rx_rate(&self, chan:usize) -> Result<f64, &'static str> {
		let mut ans:f64 = 0.0;
		match unsafe { uhd_usrp_get_rx_rate(self.handle, chan, &mut ans) } {
			0 => Ok(ans),
			_ => Err("Unable to get RX rate")
		}
	}


}