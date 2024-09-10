
use clap::{Arg, App};

use uhd_rs::usrp::USRP;

use uhd_rs::types::{TuneRequest, TuneRequestPolicy};
use std::ffi::CString;

fn main() -> Result<(), &'static str> {

	let matches = App::new("Rx Example for UHD_rs")
		.version("0.1.0")
		.author("John Stanford (johnwstanford@gmail.com)")
		.about("Records raw IQ samples to a file")
		.arg(Arg::with_name("filename")
			.short("f").long("filename")
			.help("Output filename")
			.required(false).takes_value(true))
		.arg(Arg::with_name("sample_rate_sps")
			.short("s").long("sample_rate_sps")
			.takes_value(true).required(true))
		.arg(Arg::with_name("freq_hz")
			.long("freq_hz")
			.takes_value(true).required(true))
		.arg(Arg::with_name("gain_db")
			.long("gain_db")
			.takes_value(true))
		.arg(Arg::with_name("args")
			.long("args")
			.takes_value(true))
		.arg(Arg::with_name("time_sec")
			.long("time_sec")
			.help("Time to capture [seconds]")
			.takes_value(true).required(true))
		.get_matches();

	let rx_freq = matches.value_of("freq_hz").unwrap().parse().unwrap();
	let rx_rate = matches.value_of("sample_rate_sps").unwrap().parse().unwrap();
	let rx_gain = matches.value_of("gain_db").unwrap_or("60.0").parse().unwrap();
	let rx_time = matches.value_of("time_sec").unwrap().parse::<f64>().unwrap();
	let channel = 0;

	let num_rx_samps = (rx_time * rx_rate) as usize;

	let mut usrp = USRP::new(matches.value_of("args").unwrap_or(""))?;

	println!("Clock source: {}", usrp.get_clock_source(0)?);

	// Set up RX
	let empty_args = CString::new("").unwrap();

	let tune_request = TuneRequest {
		target_freq:    rx_freq,					// Target frequency for RF chain in Hz
		rf_freq_policy: TuneRequestPolicy::Auto, 	// RF frequency policy
		rf_freq: 		0.0,						// RF frequency in Hz
		dsp_freq_policy:TuneRequestPolicy::Auto, 	// DSP frequency policy
		dsp_freq:		0.0,						// DSP frequency in Hz
		args:empty_args.as_ptr()					// Key-value pairs delimited by commas
	};

	usrp.set_rx_rate(rx_rate, channel)?;
	usrp.set_rx_gain(rx_gain, channel, "")?;
	let _rx_tune_result = usrp.set_rx_freq(&tune_request, channel)?;

	println!("RX: {:.2e} [sps], {:.1} [dB], {:.3} [MHz]", usrp.get_rx_rate(channel)?, usrp.get_rx_gain(channel, "")?, usrp.get_rx_freq(channel)? / 1.0e6);

	let mut rx_streamer = usrp.start_continuous_stream("")?;
	let mut rx_buffer:Vec<(i16, i16)> = vec![(0,0); num_rx_samps];
	let (_, rx_time_spec) = rx_streamer.read_sc16(&mut rx_buffer, None).map_err(|_| "Unable to read samples from RX streamer")?;

	println!("RX started at {:?}", rx_time_spec);

	let filename = matches.value_of("filename")
		.map(|s| s.to_owned())
		.unwrap_or(format!("rx_A0_{:.2}MHz_{}dB_{}Msps.bin", rx_freq/1.0e6, rx_gain as usize, (rx_rate/1.0e6) as usize));

	uhd_rs::io::write_sc16_to_file(filename, &rx_buffer)?;

 	Ok(())
}
