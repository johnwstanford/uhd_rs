
use clap::{Arg, App};

use uhd_rs::usrp::USRP;

fn main() -> Result<(), &'static str> {

	let matches = App::new("get_device_info")
		.version("0.1.0")
		.author("John Stanford (johnwstanford@gmail.com)")
		.about("A utility for getting USRP device information");

	let usrp = USRP::new("")?;

	println!("TX Antennas (CH0): {:?}", usrp.get_tx_antennas(0));

 	Ok(())
}