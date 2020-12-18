
use std::f64::consts::PI;
use std::ffi::CString;
use std::time::Duration;

use uhd_rs::types::{TuneRequest, TuneRequestPolicy};
use uhd_rs::usrp::USRP;

fn main() -> Result<(), &'static str> {

	// TX parameters
	let tx_freq0:f64 = 2414.0e6;
	let tx_freq1:f64 = 2414.5e6;

	let tx_rate:f64 = 1.0e6;
	let tx_gain:f64 = 60.0;

	let tx_mod_freq_hz:f64  = 0.5;
	let tx_mod_width_hz:f64 = 5.0e4;
	let tx_time_sec:f64     = 10.0;
	let num_tx_samps:usize  = (tx_rate*tx_time_sec) as usize;

	let channel = 0;

	let empty_args = CString::new("").unwrap();

	let mut usrp = USRP::new("")?;

	println!("TX num channels: {}", usrp.tx_num_channels()?);

	// Set up TX
	let tune_request0 = TuneRequest {
	    target_freq:    tx_freq0,					// Target frequency for RF chain in Hz
	    rf_freq_policy: TuneRequestPolicy::Auto, 	// RF frequency policy
	    rf_freq: 		0.0,						// RF frequency in Hz
	    dsp_freq_policy:TuneRequestPolicy::Auto, 	// DSP frequency policy
	    dsp_freq:		0.0,						// DSP frequency in Hz
	    args:empty_args.as_ptr()					// Key-value pairs delimited by commas		
	};

	usrp.set_tx_rate(tx_rate, channel)?;
	usrp.set_tx_gain(tx_gain, channel, "")?;
	let tx_tune_result0 = usrp.set_tx_freq(&tune_request0, channel)?;
	println!("{:#?}", tx_tune_result0);

	println!("TX: {:.2e} [sps], {:.1} [dB], {:.3} [MHz]", usrp.get_tx_rate(channel)?, usrp.get_tx_gain(channel, "")?, usrp.get_tx_freq(channel)? / 1.0e6);

	// Create stream
	let mut tx_streamer = usrp.get_tx_stream::<i16, i16>("")?;

	println!("Spawning TX thread");
	let tx_handle = std::thread::spawn(move || {
		let mut buffer:[(i16, i16); 1024] = [(0, 0); 1024];

		// Set up waveform
		let mut phase:f64 = 0.0;
		let mut t:f64 = 0.0;
		let dt:f64 = 1.0 / tx_rate;
		let mod_freq_rad_per_sec:f64  = tx_mod_freq_hz * 2.0 * PI;
		let mod_width_rad_per_sec:f64 = tx_mod_width_hz * 2.0 * PI;

		let mut samps_sent:usize = 0;

		while samps_sent < num_tx_samps {
			let omega:f64 = mod_width_rad_per_sec * (mod_freq_rad_per_sec*t).cos();

			t += dt*(buffer.len() as f64);

			for i in 0..buffer.len() {
				buffer[i] = ((phase.cos()*32768.0) as i16, (phase.sin()*32768.0) as i16);
				phase += dt*omega;
			}

			tx_streamer.send_sc16(&buffer, None).unwrap();
			samps_sent += buffer.len();
		}
	});

	std::thread::sleep(Duration::from_millis(4000));

	let tune_request1 = TuneRequest {
	    target_freq:    tx_freq1,					// Target frequency for RF chain in Hz
	    rf_freq_policy: TuneRequestPolicy::Auto, 	// RF frequency policy
	    rf_freq: 		0.0,						// RF frequency in Hz
	    dsp_freq_policy:TuneRequestPolicy::Auto, 	// DSP frequency policy
	    dsp_freq:		0.0,						// DSP frequency in Hz
	    args:empty_args.as_ptr()					// Key-value pairs delimited by commas		
	};

	let tx_tune_result1 = usrp.set_tx_freq(&tune_request1, channel)?;
	println!("{:#?}", tx_tune_result1);

	println!("Waiting on TX thread");
	tx_handle.join().unwrap();


 	Ok(())
}