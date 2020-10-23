
#[link(name = "uhd")]
extern {
	fn uhd_usrp_free(uhd_usrp_handle: &mut usize);	
}

#[derive(Debug)]
pub struct USRP {
	handle:usize,
	last_commanded_rate:Option<f64>,
	last_commanded_gain:Option<f64>,
	last_commanded_bw:Option<f64>,
}

mod impl_static;
mod impl_rx;
mod impl_tx;

impl std::ops::Drop for USRP {

	fn drop(&mut self) { 
		// TODO: consider checking the return value; right now we're not
		unsafe { uhd_usrp_free(&mut self.handle); } 
	}

}