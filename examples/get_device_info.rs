
use clap::App;

use uhd_rs::usrp::USRP;

fn main() -> Result<(), &'static str> {

	App::new("get_device_info")
		.version("0.1.0")
		.author("John Stanford (johnwstanford@gmail.com)")
		.about("A utility for getting USRP device information");

	let devices:Vec<String> = USRP::find("")?;
	println!("Found {} USRP Device(s)", devices.len());

	for device in devices {
		println!("Getting info for device {}", device);		
		let usrp = USRP::new(&device)?;

		let rx_info = usrp.get_rx_info(0)?;
		println!("RX Info");
		println!("  Motherboard ID: {}", rx_info.mboard_id()?);
		println!("  Motherboard Name: {}", rx_info.mboard_name()?);
		println!("  Motherboard Serial: {}", rx_info.mboard_serial()?);
		println!("  RX ID: {}", rx_info.rx_id()?);
		println!("  Subdevice name: {}", rx_info.rx_subdev_name()?);
		println!("  Subdevice spec: {}", rx_info.rx_subdev_spec()?);
		println!("  Serial: {}", rx_info.rx_serial()?);
		println!("  Antenna: {}", rx_info.rx_antenna()?);
	
		println!("TX Info");
		println!("  Antennas (CH0): {:?}", usrp.get_tx_antennas(0)?);
	}

 	Ok(())
}