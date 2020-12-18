
use std::f64::consts::PI;
use std::ffi::CString;

use clap::{Arg, App};

use uhd_rs::types::{TuneRequest, TuneRequestPolicy};
use uhd_rs::usrp::USRP;

fn main() -> Result<(), &'static str> {

	let matches = App::new("Tx Example for UHD_rs")
		.version("0.1.0")
		.author("John Stanford (johnwstanford@gmail.com)")
		.about("Transmits a simple FM sine waveform over UHD")
		.arg(Arg::with_name("sample_rate_sps")
			.short("s").long("sample_rate_sps")
			.takes_value(true))
		.arg(Arg::with_name("freq_hz")
			.long("freq_hz")
			.takes_value(true))
		.arg(Arg::with_name("gain_db")
			.long("gain_db")
			.takes_value(true))
		.arg(Arg::with_name("fm_freq_hz")
			.long("fm_freq_hz")
			.takes_value(true))
		.arg(Arg::with_name("fm_width_hz")
			.long("fm_width_hz")
			.takes_value(true))
		.arg(Arg::with_name("time_sec")
			.long("time_sec")
			.takes_value(true))
		.get_matches();

	let freq:f64 = matches.value_of("freq_hz").unwrap_or("2414e6").parse().unwrap();
	let rate:f64 = matches.value_of("sample_rate_sps").unwrap_or("4e6").parse().unwrap();
	let gain:f64 = matches.value_of("gain_db").unwrap_or("76.0").parse().unwrap();

	let mod_freq_hz:f64  = matches.value_of("fm_freq_hz").unwrap_or("0.5").parse().unwrap();
	let mod_width_hz:f64 = matches.value_of("fm_width_hz").unwrap_or("5.0e4").parse().unwrap();
	let time_sec:f64     = matches.value_of("time_sec").unwrap_or("10").parse().unwrap();
	let num_samps:usize  = (rate*time_sec) as usize;

	let channel = 0;

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

	println!("Setting TX rate: {:.2e}...", rate);
	usrp.set_tx_rate(rate, channel)?;

	println!("Actual TX rate: {:.2e}...", usrp.get_tx_rate(channel)?);

	println!("Setting TX gain: {:.2} [dB]...", gain);
	usrp.set_tx_gain(gain, channel, "")?;

	println!("Actual TX Gain: {:.2} dB...", usrp.get_tx_gain(channel, "")?);

	println!("Setting TX frequency: {:.3} [MHz]...", tune_request.target_freq / 1.0e6);
	let _tune_result = usrp.set_tx_freq(&tune_request, channel)?;

	println!("Actual TX frequency: {:.3} [MHz]...", usrp.get_tx_freq(channel)? / 1.0e6);

	// Create stream
	let mut tx_streamer = usrp.get_tx_stream::<i16, i16>("")?;
	let mut buffer:[(i16, i16); 1024] = [(0, 0); 1024];

	// Set up waveform
	let mut phase:f64 = 0.0;
	let mut t:f64 = 0.0;
	let dt:f64 = 1.0 / rate;
	let mod_freq_rad_per_sec:f64  = mod_freq_hz * 2.0 * PI;
	let mod_width_rad_per_sec:f64 = mod_width_hz * 2.0 * PI;

	let mut samps_sent:usize = 0;

	let t0_sec:i64 = 5;
	println!("Start transmission at {} [sec] USRP time", t0_sec);

	while samps_sent < num_samps {
		let omega:f64 = mod_width_rad_per_sec * (mod_freq_rad_per_sec*t).cos();

		t += dt*(buffer.len() as f64);

		for i in 0..buffer.len() {
			buffer[i] = ((phase.cos()*32768.0) as i16, (phase.sin()*32768.0) as i16);
			phase += dt*omega;
		}

		let full_secs:i64 = t as i64;
		let frac_secs:f64 = t - (full_secs as f64);

		tx_streamer.send_sc16(&buffer, Some((full_secs + t0_sec, frac_secs)))?;
		samps_sent += buffer.len();
	}
	println!("Complete at {:?} USRP time", usrp.get_time_now(0));


 	Ok(())
}