
use std::ffi::CString;
use std::io::Read;

use serde::{Serialize, Deserialize};

use crate::types::{TuneRequest, TuneRequestPolicy};
use crate::usrp::USRP;

use crate::job::Job;

const DEFAULT_CHANNEL:usize = 0;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleRx {
	pub center_freq_hz: f64,
	pub sample_rate_sps: f64,
	pub bandwidth_hz: f64,
	pub gain_db: f64,
	pub time_warmup_sec: f64,
	pub time_sec: f64,
}

impl Job<Vec<u8>> for SimpleRx {

	fn descriptor(&self) -> String {
		format!("{:.4}MHz_bw{:.1}MHz_{:.1}Msps_gain{:.1}dB_sc16", 
					self.center_freq_hz/1.0e6, 
					self.bandwidth_hz/1.0e6,
					self.sample_rate_sps/1.0e6, 
					self.gain_db)
	}

	fn sample_rate_sps(&self) -> f64 { self.sample_rate_sps }

	fn execute(&self, usrp:&mut USRP) -> Result<Vec<u8>, &'static str> {

		self.configure(usrp)?;

		usrp.start_continuous_stream::<i16, i16>("")?;

		let warmup_samps:usize = (self.time_warmup_sec * self.sample_rate_sps) as usize;
		let warmup_bytes:usize = warmup_samps * 4;
		let mut warmup_waveform:Vec<u8> = vec![0u8; warmup_bytes];
		usrp.read_exact(&mut warmup_waveform).map_err(|_| "Unable to read warmup waveform from RX streamer")?;

		let total_samps:usize = (self.time_sec * self.sample_rate_sps) as usize;
		let total_bytes:usize = total_samps * 4;
		let mut waveform:Vec<u8> = vec![0u8; total_bytes];
		usrp.read_exact(&mut waveform).map_err(|_| "Unable to read waveform from RX streamer")?;

		usrp.stop_continuous_stream()?;

		Ok(waveform)
	}


}

impl SimpleRx {

	fn configure(&self, usrp:&mut USRP) -> Result<(), &'static str> {
		
		let args = CString::new("").unwrap();

		let tune_request = TuneRequest {
		    target_freq:    self.center_freq_hz,		// Target frequency for RF chain in Hz
		    rf_freq_policy: TuneRequestPolicy::Auto, 	// RF frequency policy
		    rf_freq: 		0.0,						// RF frequency in Hz
		    dsp_freq_policy:TuneRequestPolicy::Auto, 	// DSP frequency policy
		    dsp_freq:		0.0,						// DSP frequency in Hz
		    args:args.as_ptr()							// Key-value pairs delimited by commas		
		};

		usrp.set_rx_rate(self.sample_rate_sps, DEFAULT_CHANNEL)?;
		usrp.set_rx_gain(self.gain_db, DEFAULT_CHANNEL, "")?;
		usrp.set_rx_bandwidth(self.bandwidth_hz, DEFAULT_CHANNEL)?;
		
		// TODO: consider saving the tune result
		let _tune_result = usrp.set_rx_freq(&tune_request, DEFAULT_CHANNEL)?;

		Ok(())
	}

}

