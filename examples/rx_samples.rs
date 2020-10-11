
use std::io::{Read, Write};
use std::ffi::CString;

use clap::{Arg, App};

use uhd_rs::ffi::types::{TuneRequest, TuneRequestPolicy};
use uhd_rs::ffi::usrp::StreamCmd;
use uhd_rs::usrp::USRP;

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
		.arg(Arg::with_name("time_sec")
			.long("time_sec")
			.help("Time to capture [seconds]")
			.takes_value(true).required(true))
		.get_matches();

	let freq = matches.value_of("freq_hz").unwrap().parse().unwrap();
	let rate = matches.value_of("sample_rate_sps").unwrap().parse().unwrap();
	let gain = matches.value_of("gain_db").unwrap().parse().unwrap();

	let channel = 0;

	let time:f64 = matches.value_of("time_sec").unwrap().parse().unwrap();
    let n_samples:usize = (rate*time) as usize;

	let args = CString::new("").unwrap();

	let mut usrp = USRP::new("")?;

	let tune_request = TuneRequest {
	    target_freq:    freq,						// Target frequency for RF chain in Hz
	    rf_freq_policy: TuneRequestPolicy::Auto, 	// RF frequency policy
	    rf_freq: 		0.0,						// RF frequency in Hz
	    dsp_freq_policy:TuneRequestPolicy::Auto, 	// DSP frequency policy
	    dsp_freq:		0.0,						// DSP frequency in Hz
	    args:args.as_ptr()							// Key-value pairs delimited by commas		
	};

	println!("Setting RX rate: {:.2e}...", rate);
	usrp.set_rx_rate(rate, channel)?;

	println!("Actual RX rate: {:.2e}...", usrp.get_rx_rate(channel)?);

	println!("Setting RX gain: {:.2} [dB]...", gain);
	usrp.set_rx_gain(gain, channel, "")?;

	println!("Actual RX Gain: {:.2} dB...", usrp.get_rx_gain(channel, "")?);

	println!("Setting RX frequency: {:.3} [MHz]...", tune_request.target_freq / 1.0e6);
	let _tune_result = usrp.set_rx_freq(&tune_request, channel)?;

	println!("Actual RX frequency: {:.3} [MHz]...", usrp.get_rx_freq(channel)? / 1.0e6);

	// Create stream
	let file_fmt:Option<&str> = matches.value_of("file_format");
	let (bytes_per_sample, mut rx_streamer) = match file_fmt {
		Some("sc16") => (4, usrp.get_rx_stream::<i16, i16>("")?),
		Some("fc32") => (8, usrp.get_rx_stream::<i16, f32>("")?),
		_ => return Err("Unrecognized file format")
	};

	let mut outfile = {
		let name = matches.value_of("filename")
			.map(|s| s.to_owned())
			.unwrap_or(format!("output_{:.2}MHz_{:.1}Msps_gain{:.1}dB_{}.dat", freq/1.0e6, rate/1.0e6, gain, file_fmt.unwrap()));
		std::fs::File::create(name).unwrap()
	};

	// Create stream_cmds
	let stream_cmd_start = StreamCmd::start_continuous_now();
	let stream_cmd_stop  = StreamCmd::stop_continuous_now();

	rx_streamer.stream(&stream_cmd_start)?;

	let mut total_samps:usize = 0;
	let mut buffer:[u8; 9000] = [0u8; 9000];

	while total_samps < n_samples {

		let bytes_read = rx_streamer.read(&mut buffer).map_err(|_| "Unable to read from RX streamer")?;

		rx_streamer.rx_metadata_ok()?;

		if bytes_read > 0 {

			outfile.write(&buffer[..bytes_read]).unwrap();

			total_samps += bytes_read / bytes_per_sample;
		}
	}

	rx_streamer.stream(&stream_cmd_stop)?;

	let (full_secs, frac_secs) = rx_streamer.rx_metadata_time_spec()?;
	eprintln!("Last received packet time: {} full secs, {} frac secs", full_secs, frac_secs);

 	Ok(())
}