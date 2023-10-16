
use std::ffi::CString;

use libc::{c_char, size_t};
use crate::c_interop::{collect_cstr, populate_cstr};

use crate::check_err;
use crate::rx_streamer::RxStreamer;
use crate::types::{TuneRequest, TuneResult, TuneRequestPolicy};
use crate::types::string_vector::StringVector;
use crate::types::usrp_info::Info;
use crate::usrp::{StreamArgs, StreamCmd};
use crate::usrp::subdev_spec::SubdevSpec;

#[link(name = "uhd")]
extern {

	// uhd_error uhd_usrp_set_rx_subdev_spec(uhd_usrp_handle h, uhd_subdev_spec_handle subdev_spec, size_t mboard)
	fn uhd_usrp_get_rx_subdev_spec(h: usize, mboard: usize, subdev_spec_out: usize) -> isize;
	fn uhd_usrp_get_rx_subdev_name(h: usize, chan: size_t, rx_subdev_name_out: *mut u8, strbuffer_len: size_t) -> isize;

	// uhd_error uhd_usrp_get_rx_freq_range(uhd_usrp_handle h, size_t chan, uhd_meta_range_handle freq_range_out)
	// uhd_error uhd_usrp_get_fe_rx_freq_range(uhd_usrp_handle h, size_t chan, uhd_meta_range_handle freq_range_out)

	fn uhd_usrp_get_rx_lo_names(h: usize, chan: size_t, rx_lo_names_out: *mut usize) -> isize;
	// uhd_error uhd_usrp_set_rx_lo_source(uhd_usrp_handle h, const char* src, const char* name, size_t chan)
	fn uhd_usrp_get_rx_lo_source(h: usize, name: *const u8, chan: size_t, rx_lo_source_out: *mut u8, strbuffer_len: size_t) -> isize;
	fn uhd_usrp_get_rx_lo_sources(h: usize, name: *const u8, chan: size_t, rx_lo_sources_out: *mut usize) -> isize;
	// uhd_error uhd_usrp_set_rx_lo_export_enabled(uhd_usrp_handle h, bool enabled, const char* name, size_t chan)
	// uhd_error uhd_usrp_get_rx_lo_export_enabled(uhd_usrp_handle h, const char* name, size_t chan, bool* result_out)

	// uhd_error uhd_usrp_set_rx_lo_freq(uhd_usrp_handle h, double freq, const char* name, size_t chan, double* coerced_freq_out)
	// uhd_error uhd_usrp_get_rx_lo_freq(uhd_usrp_handle h, const char* name, size_t chan, double* rx_lo_freq_out)
	// uhd_error uhd_usrp_set_normalized_rx_gain(uhd_usrp_handle h, double gain, size_t chan)
	// uhd_error uhd_usrp_set_rx_agc(uhd_usrp_handle h, bool enable, size_t chan)
	// uhd_error uhd_usrp_get_normalized_rx_gain(uhd_usrp_handle h, size_t chan, double *gain_out)
	// uhd_error uhd_usrp_get_rx_gain_range(uhd_usrp_handle h, const char* name, size_t chan, uhd_meta_range_handle gain_range_out)
	// uhd_error uhd_usrp_get_rx_gain_names(uhd_usrp_handle h, size_t chan, uhd_string_vector_handle *gain_names_out)
	// uhd_error uhd_usrp_set_rx_antenna(uhd_usrp_handle h, const char* ant, size_t chan)
	// uhd_error uhd_usrp_get_rx_antenna(uhd_usrp_handle h, size_t chan, char* ant_out, size_t strbuffer_len)

	// uhd_error uhd_usrp_get_rx_sensor_names(uhd_usrp_handle h, size_t chan, uhd_string_vector_handle *sensor_names_out)
	// uhd_error uhd_usrp_get_rx_bandwidth_range(uhd_usrp_handle h, size_t chan, uhd_meta_range_handle bandwidth_range_out)
	// uhd_error uhd_usrp_get_rx_sensor(uhd_usrp_handle h, const char* name, size_t chan, uhd_sensor_value_handle *sensor_value_out)
	// uhd_error uhd_usrp_set_rx_dc_offset_enabled(uhd_usrp_handle h, bool enb, size_t chan)
	// uhd_error uhd_usrp_set_rx_iq_balance_enabled(uhd_usrp_handle h, bool enb, size_t chan)

	fn uhd_usrp_get_rx_info(h:usize, chan:size_t, info_out:&mut Info) -> isize;
	fn uhd_usrp_get_rx_num_channels(h:usize, num_channels_out:&mut size_t) -> isize;

	fn uhd_usrp_get_rx_antennas(h:usize, chan:size_t, uhd_string_vector_handle:&mut usize) -> isize;

	fn uhd_usrp_set_rx_rate(h:usize, rate:f64, chan:size_t) -> isize;
	fn uhd_usrp_get_rx_rate(h:usize, chan:size_t, rate_out:&mut f64) -> isize;
	fn uhd_usrp_set_rx_gain(h:usize, gain:f64, chan:size_t, gain_name:*const c_char) -> isize;
	fn uhd_usrp_get_rx_gain(h:usize, chan:size_t, gain_name:*const c_char, gain_out:&mut f64) -> isize;
	fn uhd_usrp_set_rx_freq(h:usize, tune_request:&TuneRequest, chan:size_t, tune_result:&mut TuneResult) -> isize;
	fn uhd_usrp_get_rx_freq(h:usize, chan:size_t, freq_out:&mut f64) -> isize;
	fn uhd_usrp_set_rx_bandwidth(h:usize, bandwidth:f64, chan:size_t) -> isize;
	fn uhd_usrp_get_rx_bandwidth(h:usize, chan:size_t, bandwidth_out:&mut f64) -> isize;

	fn uhd_usrp_get_rx_stream(h:usize, stream_args:&StreamArgs, h_out:usize) -> isize;

}

impl super::USRP {

	pub fn get_rx_lo_source(&self, name: &str, chan: usize) -> Result<String, &'static str> {
		let mut name_buff: Vec<u8> = vec![0; 64];
		let mut source_buff: Vec<u8> = vec![0; 64];
		unsafe {
			populate_cstr(name_buff.as_mut_ptr(), name_buff.len(), name);
			let result = uhd_usrp_get_rx_lo_source(self.handle, name_buff.as_ptr(), chan, source_buff.as_mut_ptr(), source_buff.len());
			match result {
				0 => Ok(collect_cstr(source_buff.as_ptr())),
				_ => {
					eprintln!("{:?}", self.last_error());
					Err("Unable to get LO source")
				}
			}
		}
	}

	pub fn get_rx_lo_names(&self, chan: usize) -> Result<StringVector, &'static str> {
		let mut sv = StringVector::new()?;
		check_err((), unsafe {
			uhd_usrp_get_rx_lo_names(self.handle, chan, &mut sv.handle)
		})?;
		Ok(sv)
	}

	pub fn get_rx_lo_sources(&self, name: &str, chan: usize) -> Result<StringVector, &'static str> {
		let mut sv = StringVector::new()?;
		let mut name_buff: Vec<u8> = vec![0; 64];
		check_err((), unsafe {
			populate_cstr(name_buff.as_mut_ptr(), name_buff.len(), name);
			uhd_usrp_get_rx_lo_sources(self.handle, name_buff.as_ptr(), chan, &mut sv.handle)
		})?;
		Ok(sv)
	}

	pub fn get_subdev_spec(&self, mboard: usize) -> Result<SubdevSpec, &'static str> {
		let spec = SubdevSpec::new("A0")?;
		unsafe {
			match uhd_usrp_get_rx_subdev_spec(self.handle, mboard, spec.handle) {
				0 => Ok(spec),
				_ => Err("USRP::get_subdev_spec failed")
			}
		}
	}

	pub fn get_rx_subdev_name(&self, chan: usize) -> Result<String, &'static str> {
		let mut buff: Vec<u8> = vec![0; 128];
		unsafe {
			let err = uhd_usrp_get_rx_subdev_name(
				self.handle, chan,
				buff.as_mut_ptr(), buff.len());

			match err {
				0 => Ok(collect_cstr(buff.as_ptr())),
				_ => Err("Nonzero return value from get_rx_subdev_name"),
			}

		}

	}

	pub fn get_rx_bandwidth(&self, chan:usize) -> Result<f64, &'static str> {
		let mut ans:f64 = 0.0;
		let result = unsafe { uhd_usrp_get_rx_bandwidth(self.handle, chan, &mut ans) };
		check_err(ans, result)
	}

	pub fn set_rx_bandwidth(&mut self, bandwidth:f64, chan:usize) -> Result<(), &'static str> {
		check_err((), unsafe { uhd_usrp_set_rx_bandwidth(self.handle, bandwidth, chan) })
	}

	pub fn start_continuous_stream(&mut self, args:&str) -> Result<RxStreamer, &'static str> {
		
		let mut rx_streamer = self.get_rx_stream(args, &[0])?;

		let stream_cmd_start = StreamCmd::start_continuous_now();
		rx_streamer.stream(&stream_cmd_start)?;

		Ok(rx_streamer)
	}

	pub fn get_rx_stream(&mut self, args:&str, chans: &[size_t]) -> Result<RxStreamer, &'static str> {
		// Note: This implementation assumes that you always want to create a new RxStreamer for every stream you want
		// to create.  If you're going to be creating and destroying streams all the time, it might be more efficient to
		// reuse instances of an RxStreamer.  If that ends up being the case, we could potentially create some kind of 
		// pool of them inside the USRP struct and still provide the same abstraction to the outside
		let otw_format = CString::new("sc16").unwrap();
		let cpu_format = CString::new("sc16").unwrap();

		let args_cstr = CString::new(args).unwrap();

		let stream_args = StreamArgs {
		    cpu_format:cpu_format.as_ptr(),		// Format of host memory
		    otw_format:otw_format.as_ptr(),		// Over-the-wire format
		    args:args_cstr.as_ptr(),			// Other stream args
		    channel_list: chans.as_ptr(),   	// Array that lists channels
		    n_channels: chans.len() as isize	// Number of channels
		};

		let mut rx_streamer = RxStreamer::new(stream_args.n_channels as usize)?;
		if unsafe { uhd_usrp_get_rx_stream(self.handle, &stream_args, rx_streamer.get_handle()) } != 0 {
			return Err("Unable to get an RxStream");
		}

		rx_streamer.get_max_num_samps()?;

		Ok(rx_streamer)
	}

	// Get information
	pub fn rx_num_channels(&self) -> Result<usize, &'static str> {
		let mut ans = 0;
		let result = unsafe { uhd_usrp_get_rx_num_channels(self.handle, &mut ans) };
		check_err(ans, result)
	}

	pub fn get_rx_info(&self, chan:usize) -> Result<Info, &'static str> {
		let mut ans = Info::null();
		let result = unsafe { uhd_usrp_get_rx_info(self.handle, chan, &mut ans) };
		check_err(ans, result)
	}

	pub fn get_rx_antennas(&self, chan:usize) -> Result<Vec<String>, &'static str> {
		let mut string_vec = StringVector::new()?;
		let result = unsafe { uhd_usrp_get_rx_antennas(self.handle, chan, &mut string_vec.handle) };
		check_err(string_vec.get_rust_vec()?, result)
	} 

	// Get or set configuration values
	pub fn set_rx_rate(&mut self, rate:f64, chan:usize) -> Result<(), &'static str> {
		check_err((), unsafe { uhd_usrp_set_rx_rate(self.handle, rate, chan) })
	}

	pub fn get_rx_rate(&self, chan:usize) -> Result<f64, &'static str> {
		let mut ans:f64 = 0.0;
		let result = unsafe { uhd_usrp_get_rx_rate(self.handle, chan, &mut ans) };
		check_err(ans, result)
	}

	pub fn set_rx_gain(&mut self, gain:f64, chan:usize, gain_name:&str) -> Result<(), &'static str> {
		let gain_name_c:CString = CString::new(gain_name).unwrap();
		check_err((), unsafe { uhd_usrp_set_rx_gain(self.handle, gain, chan, gain_name_c.as_ptr()) })
	}

	pub fn get_rx_gain(&self, chan:usize, gain_name:&str) -> Result<f64, &'static str> {
		let gain_name_c:CString = CString::new(gain_name).unwrap();
		let mut gain_out:f64 = 0.0;
		let result = unsafe { uhd_usrp_get_rx_gain(self.handle, chan, gain_name_c.as_ptr(), &mut gain_out) };
		check_err(gain_out, result)
	}

	pub fn set_rx_freq(&mut self, tune_request:&TuneRequest, chan:usize) -> Result<TuneResult, &'static str> {
		let mut tune_result:TuneResult = TuneResult::default();
		let result = unsafe { uhd_usrp_set_rx_freq(self.handle, tune_request, chan, &mut tune_result) };
		check_err(tune_result, result)
	}

	pub fn set_rx_freq_auto(&mut self, freq_hz:f64, chan:usize) -> Result<TuneResult, &'static str> {
		let args = CString::new("").unwrap();
		let tune_request = TuneRequest {
		    target_freq:    freq_hz,					// Target frequency for RF chain in Hz
		    rf_freq_policy: TuneRequestPolicy::Auto, 	// RF frequency policy
		    rf_freq: 		0.0,						// RF frequency in Hz
		    dsp_freq_policy:TuneRequestPolicy::Auto, 	// DSP frequency policy
		    dsp_freq:		0.0,						// DSP frequency in Hz
		    args:args.as_ptr()							// Key-value pairs delimited by commas		
		};
		self.set_rx_freq(&tune_request, chan)
	}

	pub fn get_rx_freq(&self, chan:usize) -> Result<f64, &'static str> {
		let mut freq_out:f64 = 0.0;
		let result = unsafe { uhd_usrp_get_rx_freq(self.handle, chan, &mut freq_out) };
		check_err(freq_out, result)
	}


}