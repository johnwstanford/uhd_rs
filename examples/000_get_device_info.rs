
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

		let num_mboards:usize = usrp.num_mboards()?;
		println!("Number of motherboard(s): {}", num_mboards);	
		for mboard_idx in 0..num_mboards {
			println!("Motherboard {}:", mboard_idx);
			println!(" Time source:  {} ({:?})", usrp.get_time_source(mboard_idx)?, usrp.get_time_sources(mboard_idx)?);
			println!(" Clock source: {} ({:?})", usrp.get_clock_source(mboard_idx)?, usrp.get_clock_sources(mboard_idx)?);
		}

		println!("RX Channels");
		for chan in 0..(usrp.rx_num_channels()?) {
			let rx_info = usrp.get_rx_info(chan)?;
			println!(" RX Channel {}", chan);
			println!("  Motherboard ID: {}", rx_info.mboard_id()?);
			println!("  Motherboard Name: {}", rx_info.mboard_name()?);
			println!("  Motherboard Serial: {}", rx_info.mboard_serial()?);
			println!("  RX ID: {}", rx_info.id()?);
			println!("  Subdevice name: {}", rx_info.subdev_name()?);
			println!("  Subdevice spec: {}", rx_info.subdev_spec()?);
			println!("  Serial: {}", rx_info.serial()?);
			println!("  Antenna: {} ({:?})", rx_info.antenna()?, usrp.get_rx_antennas(chan)?);
		}

		let tx_info = usrp.get_tx_info(0)?;
		println!("TX Info");
		println!("  Motherboard ID: {}", tx_info.mboard_id()?);
		println!("  Motherboard Name: {}", tx_info.mboard_name()?);
		println!("  Motherboard Serial: {}", tx_info.mboard_serial()?);
		println!("  TX ID: {}", tx_info.id()?);
		println!("  Subdevice name: {}", tx_info.subdev_name()?);
		println!("  Subdevice spec: {}", tx_info.subdev_spec()?);
		println!("  Serial: {}", tx_info.serial()?);
		println!("  Antenna: {}", tx_info.antenna()?);

		println!("  Num channels: {}", usrp.tx_num_channels()?);
	}

 	Ok(())
}