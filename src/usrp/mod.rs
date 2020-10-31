
use libc::size_t;

use crate::rx_streamer::RxStreamer;

#[link(name = "uhd")]
extern {

	fn uhd_usrp_get_num_mboards(h:usize, num_mboards_out:&mut size_t) -> isize;

	fn uhd_usrp_free(uhd_usrp_handle: &mut usize);	
}

pub struct USRP {
	handle:usize,
	last_commanded_rate:Option<f64>,
	last_commanded_gain:Option<f64>,
	last_commanded_bw:Option<f64>,
	opt_rx_streamer:Option<RxStreamer>,
}

mod impl_static;
mod impl_rx;
mod impl_tx;

impl USRP {

	pub fn num_mboards(&self) -> Result<usize, &'static str> {
		let mut ans = 0;
		match unsafe { uhd_usrp_get_num_mboards(self.handle, &mut ans) } {
			0 => Ok(ans),
			_ => Err("Unable to get the number of motherboards"),
		}
	}

}

impl std::ops::Drop for USRP {

	fn drop(&mut self) { 
		// TODO: consider checking the return value; right now we're not
		unsafe { uhd_usrp_free(&mut self.handle); } 
	}

}