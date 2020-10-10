
use clap::App;

use uhd_rs::usrp::USRP;

fn main() -> Result<(), &'static str> {

	let matches = App::new("get_device_info")
		.version("0.1.0")
		.author("John Stanford (johnwstanford@gmail.com)")
		.about("A utility for getting USRP device information");

	let devices:Vec<String> = USRP::find("")?;
	println!("Found {} USRP Device(s)", devices.len());

	for device in devices {
		println!("Getting info for device {}", device);		
		let usrp = USRP::new(&device)?;
	
		println!("TX Antennas (CH0): {:?}", usrp.get_tx_antennas(0));
	}

 	Ok(())
}