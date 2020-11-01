
use std::any::{Any, TypeId};
use std::ffi::CString;

use libc::{c_char, size_t};

use crate::tx_streamer::TxStreamer;
use crate::types::{TuneRequest, TuneResult};
use crate::types::string_vector::StringVector;
use crate::types::usrp_info::Info;
use crate::usrp::StreamArgs;

#[link(name = "uhd")]
extern {

	// uhd_error uhd_usrp_set_tx_subdev_spec(uhd_usrp_handle h, uhd_subdev_spec_handle subdev_spec, size_t mboard)
	// uhd_error uhd_usrp_get_tx_subdev_spec(uhd_usrp_handle h, size_t mboard, uhd_subdev_spec_handle subdev_spec_out)
	// uhd_error uhd_usrp_get_tx_subdev_name(uhd_usrp_handle h, size_t chan, char* tx_subdev_name_out, size_t strbuffer_len)
	// uhd_error uhd_usrp_get_tx_rates(uhd_usrp_handle h, size_t chan, uhd_meta_range_handle rates_out)
	// uhd_error uhd_usrp_get_tx_freq_range(uhd_usrp_handle h, size_t chan, uhd_meta_range_handle freq_range_out)
	// uhd_error uhd_usrp_get_fe_tx_freq_range(uhd_usrp_handle h, size_t chan, uhd_meta_range_handle freq_range_out)
	// uhd_error uhd_usrp_get_tx_lo_names(uhd_usrp_handle h, size_t chan, uhd_string_vector_handle *tx_lo_names_out)
	// uhd_error uhd_usrp_set_tx_lo_source(uhd_usrp_handle h, const char* src, const char* name, size_t chan)
	// uhd_error uhd_usrp_get_tx_lo_source(uhd_usrp_handle h, const char* name, size_t chan, char* tx_lo_source_out, size_t strbuffer_len)
	// uhd_error uhd_usrp_get_tx_lo_sources(uhd_usrp_handle h, const char* name, size_t chan, uhd_string_vector_handle *tx_lo_sources_out)
	// uhd_error uhd_usrp_set_tx_lo_export_enabled(uhd_usrp_handle h, bool enabled, const char* name, size_t chan)
	// uhd_error uhd_usrp_get_tx_lo_export_enabled(uhd_usrp_handle h, const char* name, size_t chan, bool* result_out)
	// uhd_error uhd_usrp_set_tx_lo_freq(uhd_usrp_handle h, double freq, const char* name, size_t chan, double* coerced_freq_out)
	// uhd_error uhd_usrp_get_tx_lo_freq(uhd_usrp_handle h, const char* name, size_t chan, double* tx_lo_freq_out)
	// uhd_error uhd_usrp_set_normalized_tx_gain(uhd_usrp_handle h, double gain, size_t chan)
	// uhd_error uhd_usrp_get_tx_gain_range(uhd_usrp_handle h, const char* name, size_t chan, uhd_meta_range_handle gain_range_out)
	// uhd_error uhd_usrp_get_normalized_tx_gain(uhd_usrp_handle h, size_t chan, double *gain_out)
	// uhd_error uhd_usrp_get_tx_gain_names(uhd_usrp_handle h, size_t chan, uhd_string_vector_handle *gain_names_out)
	// uhd_error uhd_usrp_set_tx_antenna(uhd_usrp_handle h, const char* ant, size_t chan)
	// uhd_error uhd_usrp_get_tx_antenna(uhd_usrp_handle h, size_t chan, char* ant_out, size_t strbuffer_len)
	// uhd_error uhd_usrp_set_tx_bandwidth(uhd_usrp_handle h, double bandwidth, size_t chan)
	// uhd_error uhd_usrp_get_tx_bandwidth(uhd_usrp_handle h, size_t chan, double *bandwidth_out)
	// uhd_error uhd_usrp_get_tx_bandwidth_range(uhd_usrp_handle h, size_t chan, uhd_meta_range_handle bandwidth_range_out)
	// uhd_error uhd_usrp_get_tx_sensor(uhd_usrp_handle h, const char* name, size_t chan, uhd_sensor_value_handle *sensor_value_out)
	// uhd_error uhd_usrp_get_tx_sensor_names(uhd_usrp_handle h, size_t chan, uhd_string_vector_handle *sensor_names_out)

	fn uhd_usrp_get_tx_info(h:usize, chan:size_t, info_out:&mut Info) -> isize;
	fn uhd_usrp_get_tx_antennas(h:usize, chan:size_t, antennas_out:&mut usize) -> isize;
	fn uhd_usrp_get_tx_num_channels(h:usize, num_channels_out:&mut usize) -> isize;
	
	fn uhd_usrp_set_tx_rate(h:usize, rate:f64, chan:size_t) -> isize;
	fn uhd_usrp_get_tx_rate(h:usize, chan:size_t, rate_out:&mut f64) -> isize;
	fn uhd_usrp_set_tx_freq(h:usize, tune_request:&TuneRequest, chan:size_t, tune_result:&mut TuneResult) -> isize;
	fn uhd_usrp_get_tx_freq(h:usize, chan:size_t, freq_out:&mut f64) -> isize;
	fn uhd_usrp_set_tx_gain(h:usize, gain:f64, chan:size_t, gain_name:*const c_char) -> isize;
	fn uhd_usrp_get_tx_gain(h:usize, chan:size_t, gain_name:*const c_char, gain_out:&mut f64) -> isize;
	
	fn uhd_usrp_get_tx_stream(h:usize, stream_args:&StreamArgs, h_out:usize) -> isize;

}

impl super::USRP {

	pub fn get_tx_stream<W: Any, U: Any>(&mut self, args:&str) -> Result<TxStreamer, &'static str> {

		let otw_format = match TypeId::of::<W>() {
			id if id == TypeId::of::<i16>() => CString::new("sc16").unwrap(),
			_ => return Err("Unsupported type for wire format")
		};
		let cpu_format = match TypeId::of::<U>() {
			id if id == TypeId::of::<i16>() => CString::new("sc16").unwrap(),
			id if id == TypeId::of::<f32>() => CString::new("fc32").unwrap(),
			_ => return Err("Unsupported type for CPU format")
		};

		let args_cstr = CString::new(args).unwrap();

		// We only support one channel per stream right now
		let channel = 0;

		let stream_args = StreamArgs {
		    cpu_format:cpu_format.as_ptr(),	// Format of host memory
		    otw_format:otw_format.as_ptr(),	// Over-the-wire format		
		    args:args_cstr.as_ptr(),		// Other stream args
		    channel_list:&channel, 			// Array that lists channels
		    n_channels:1					// Number of channels
		};

		let mut tx_streamer = TxStreamer::new(stream_args.n_channels as usize)?;
		if unsafe { uhd_usrp_get_tx_stream(self.handle, &stream_args, tx_streamer.get_handle()) } != 0 {
			return Err("Unable to get an TxStream");
		}

		tx_streamer.get_max_num_samps()?;

		Ok(tx_streamer)
	}

	pub fn tx_num_channels(&self) -> Result<usize, &'static str> {
		let mut ans:usize = 0;
		match unsafe { uhd_usrp_get_tx_num_channels(self.handle, &mut ans) } {
			0 => Ok(ans),
			_ => Err("Unable to get number of TX channels")
		}
	}

	pub fn get_tx_info(&self, chan:usize) -> Result<Info, &'static str> {
		let mut ans = Info::null();
		match unsafe { uhd_usrp_get_tx_info(self.handle, chan, &mut ans) } {
			0 => Ok(ans),
			_ => Err("Unable to get Tx info")
		}
	}

	pub fn get_tx_antennas(&self, chan:usize) -> Result<Vec<String>, &'static str> {
		let mut string_vec = StringVector::new()?;
		match unsafe { uhd_usrp_get_tx_antennas(self.handle, chan, &mut string_vec.handle) } {
			0 => Ok(string_vec.get_rust_vec()?),
			_ => Err("Unable to retrieve TX antennas")
		}
	}

	pub fn set_tx_rate(&mut self, rate:f64, chan:usize) -> Result<(), &'static str> {
		match unsafe { uhd_usrp_set_tx_rate(self.handle, rate, chan) } {
			0 => Ok(()),
			_ => Err("Unable to set TX rate")
		}
	}

	pub fn get_tx_rate(&self, chan:usize) -> Result<f64, &'static str> {
		let mut ans:f64 = 0.0;
		match unsafe { uhd_usrp_get_tx_rate(self.handle, chan, &mut ans) } {
			0 => Ok(ans),
			_ => Err("Unable to get TX rate")
		}
	}

	pub fn set_tx_freq(&mut self, tune_request:&TuneRequest, chan:usize) -> Result<TuneResult, &'static str> {
		let mut tune_result:TuneResult = TuneResult::default();
		match unsafe { uhd_usrp_set_tx_freq(self.handle, tune_request, chan, &mut tune_result) } {
			0 => Ok(tune_result),
			_ => Err("Unable to set TX freq")
		}
	}

	pub fn get_tx_freq(&self, chan:usize) -> Result<f64, &'static str> {
		let mut freq_out:f64 = 0.0;
		match unsafe { uhd_usrp_get_tx_freq(self.handle, chan, &mut freq_out) } {
			0 => Ok(freq_out),
			_ => Err("Unable to get TX freq")
		}
	}

	pub fn set_tx_gain(&mut self, gain:f64, chan:usize, gain_name:&str) -> Result<(), &'static str> {
		let gain_name_c:CString = CString::new(gain_name).unwrap();
		match unsafe { uhd_usrp_set_tx_gain(self.handle, gain, chan, gain_name_c.as_ptr()) } {
			0 => Ok(()),
			_ => Err("Unable to set TX gain")
		}
	}

	pub fn get_tx_gain(&self, chan:usize, gain_name:&str) -> Result<f64, &'static str> {
		let gain_name_c:CString = CString::new(gain_name).unwrap();
		let mut gain_out:f64 = 0.0;
		match unsafe { uhd_usrp_get_tx_gain(self.handle, chan, gain_name_c.as_ptr(), &mut gain_out) } {
			0 => Ok(gain_out),
			_ => Err("Unable to get TX gain")
		}
	}


}