
use clap::{Arg, App};

use uhd_rs::usrp::USRP;

use uhd_rs::job::{Job, simple_rx};

const DEFAULT_WARMUP_SEC:&str = "0.5";

fn main() -> Result<(), &'static str> {

	let matches = App::new("Rx Example for UHD_rs")
		.version("0.1.0")
		.author("John Stanford (johnwstanford@gmail.com)")
		.about("Records raw IQ samples to a file")
		.arg(Arg::with_name("filename")
			.short("f").long("filename")
			.help("Output filename")
			.required(false).takes_value(true))
		.arg(Arg::with_name("file_format")
			.long("format")
			.takes_value(true)
			.default_value("sc16")
			.possible_value("sc16")
			.possible_value("fc32"))
		.arg(Arg::with_name("sample_rate_sps")
			.short("s").long("sample_rate_sps")
			.takes_value(true).required(true))
		.arg(Arg::with_name("freq_hz")
			.long("freq_hz")
			.takes_value(true).required(true))
		.arg(Arg::with_name("gain_db")
			.long("gain_db")
			.takes_value(true).required(true))
		.arg(Arg::with_name("args")
			.long("args")
			.takes_value(true))
		.arg(Arg::with_name("warmup_time_sec")
			.long("warmup_time_sec")
			.help("Time to discard before capture [seconds]")
			.takes_value(true).required(false))
		.arg(Arg::with_name("time_sec")
			.long("time_sec")
			.help("Time to capture [seconds]")
			.takes_value(true).required(true))
		.get_matches();

	let sample_rate_sps = matches.value_of("sample_rate_sps").unwrap().parse().unwrap();

	let job = simple_rx::SimpleRx {
		sample_rate_sps, bandwidth_hz: sample_rate_sps,
		center_freq_hz:  matches.value_of("freq_hz").unwrap().parse().unwrap(),
		gain_db:         matches.value_of("gain_db").unwrap().parse().unwrap(),
		time_warmup_sec: matches.value_of("warmup_time_sec").unwrap_or(DEFAULT_WARMUP_SEC).parse().unwrap(),
		time_sec:        matches.value_of("time_sec").unwrap().parse().unwrap()
	};

	let mut usrp = USRP::new(matches.value_of("args").unwrap_or(""))?;

	println!("Clock source: {}", usrp.get_clock_source(0)?);

	let waveform:Vec<u8> = job.execute(&mut usrp)?;

	let filename = matches.value_of("filename")
		.map(|s| s.to_owned())
		.unwrap_or(format!("output_{}.dat", job.descriptor()));

	std::fs::write(filename, &waveform).map_err(|_| "Unable to save output file")?;

 	Ok(())
}