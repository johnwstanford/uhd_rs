
use clap::{Arg, App};
use std::io::BufReader;

use uhd_rs::usrp::USRP;

use uhd_rs::job::{Job, simple_rx};

const DEFAULT_WARMUP_SEC:&str = "0.5";

fn main() -> Result<(), &'static str> {

	let matches = App::new("Rx Example for UHD_rs")
		.version("0.1.0")
		.author("John Stanford (johnwstanford@gmail.com)")
		.about("Records raw IQ samples to a file")
		.arg(Arg::with_name("config_filename")
			.short("c").long("config")
			.help("Configuration YAML file")
			.required(false).takes_value(true))
		.get_matches();

	let config:Vec<simple_rx::SimpleRx> = {
		let filename = matches.value_of("config_filename")
			.map(|s| s.to_owned())
			.unwrap_or("rx_multiple.yaml".to_owned());
		let f = std::fs::File::open(filename).map_err(|_| "Unable to open configuration YAML")?;
		serde_yaml::from_reader(BufReader::new(f)).map_err(|_| "Unable to parse configuration YAML")?
	};

	println!("{:#?}", config);

	// let mut usrp = USRP::new("")?;

	// let waveform:Vec<u8> = job.execute(&mut usrp)?;


	// std::fs::write("rx_multiple.yaml", serde_yaml::to_string(&config).unwrap().as_bytes()).map_err(|_| "Unable to save output file")?;

 	Ok(())
}