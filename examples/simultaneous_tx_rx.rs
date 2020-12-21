
use std::f64::consts::PI;
use std::ffi::CString;

use uhd_rs::types::{TuneRequest, TuneRequestPolicy};
use uhd_rs::usrp::USRP;

fn main() -> Result<(), &'static str> {

	// TX parameters
	let tx_freq:f64 = 2414.0e6;
	let tx_rate:f64 = 1.0e6;
	let tx_gain:f64 = 60.0;

	let tx_mod_freq_hz:f64  = 0.5;
	let tx_mod_width_hz:f64 = 5.0e4;
	let tx_time_sec:f64     = 10.0;
	let num_tx_samps:usize  = (tx_rate*tx_time_sec) as usize;

	// RX parameters
	// Note that the sample rates don't have to be the same
	let rx_freq:f64 = 2414.1e6;
	let rx_rate:f64 = 2.0e6;
	let rx_gain:f64 = 60.0;

	let rx_time_sec:f64    = 5.0;
	let num_rx_samps:usize = (rx_rate*rx_time_sec) as usize;

	let channel = 0;

	let empty_args = CString::new("").unwrap();

	let mut usrp = USRP::new("")?;

	println!("USRP num motherboards: {}", usrp.num_mboards()?);
	println!("TX num channels: {}", usrp.tx_num_channels()?);
	println!("RX num channels: {}", usrp.rx_num_channels()?);

	{
		// Set up TX
		let tune_request = TuneRequest {
		    target_freq:    tx_freq,					// Target frequency for RF chain in Hz
		    rf_freq_policy: TuneRequestPolicy::Auto, 	// RF frequency policy
		    rf_freq: 		0.0,						// RF frequency in Hz
		    dsp_freq_policy:TuneRequestPolicy::Auto, 	// DSP frequency policy
		    dsp_freq:		0.0,						// DSP frequency in Hz
		    args:empty_args.as_ptr()					// Key-value pairs delimited by commas		
		};

		usrp.set_tx_rate(tx_rate, channel)?;
		usrp.set_tx_gain(tx_gain, channel, "")?;
		let _tx_tune_result = usrp.set_tx_freq(&tune_request, channel)?;

		println!("TX: {:.2e} [sps], {:.1} [dB], {:.3} [MHz]", usrp.get_tx_rate(channel)?, usrp.get_tx_gain(channel, "")?, usrp.get_tx_freq(channel)? / 1.0e6);
	}

	{
		// Set up RX
		let tune_request = TuneRequest {
		    target_freq:    rx_freq,					// Target frequency for RF chain in Hz
		    rf_freq_policy: TuneRequestPolicy::Auto, 	// RF frequency policy
		    rf_freq: 		0.0,						// RF frequency in Hz
		    dsp_freq_policy:TuneRequestPolicy::Auto, 	// DSP frequency policy
		    dsp_freq:		0.0,						// DSP frequency in Hz
		    args:empty_args.as_ptr()							// Key-value pairs delimited by commas		
		};

		usrp.set_rx_rate(rx_rate, channel)?;
		usrp.set_rx_gain(rx_gain, channel, "")?;		
		let _rx_tune_result = usrp.set_rx_freq(&tune_request, channel)?;

		println!("RX: {:.2e} [sps], {:.1} [dB], {:.3} [MHz]", usrp.get_rx_rate(channel)?, usrp.get_rx_gain(channel, "")?, usrp.get_rx_freq(channel)? / 1.0e6);
	}

	// Create stream
	let mut tx_streamer = usrp.get_tx_stream::<i16, i16>("")?;

	println!("Spawning TX thread at USRP time {:?}", usrp.get_time_now(0));
	let tx_handle = std::thread::spawn(move || {
		let mut buffer:[(i16, i16); 4096] = [(0, 0); 4096];

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

			tx_streamer.single_coherent_pulse(&buffer, None).unwrap();
			samps_sent += buffer.len();
		}
	});

	let mut rx_streamer = usrp.start_continuous_stream::<i16, i16>("")?;
	let mut rx_buffer:Vec<(i16, i16)> = vec![(0,0); num_rx_samps];
	let rx_time_spec = rx_streamer.read_sc16(&mut rx_buffer).map_err(|_| "Unable to read samples from RX streamer")?;
	println!("RX Time: {:?}", rx_time_spec);
	println!("RX complete at USRP time {:?}", usrp.get_time_now(0));

	let filename:String = format!("output_{:.2}MHz_{:.1}Msps_gain{:.1}dB_sc16.dat", rx_freq/1.0e6, rx_rate/1.0e6, rx_gain);
	let rx_buffer_ptr:*const u8 = &rx_buffer[0] as *const (i16,i16) as *const u8;
	let rx_bytes:&[u8] = unsafe{ std::slice::from_raw_parts(rx_buffer_ptr, rx_buffer.len() * std::mem::size_of::<(i16,i16)>()) };
	std::fs::write(&filename, rx_bytes).map_err(|_| "Unable to save output file")?;

	println!("Waiting on TX thread");
	tx_handle.join().unwrap();
	println!("TX thread complete at USRP time {:?}", usrp.get_time_now(0));

 	Ok(())
}