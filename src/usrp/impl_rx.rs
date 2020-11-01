
use std::any::{Any, TypeId};
use std::ffi::CString;

use libc::{c_char, size_t};

use crate::rx_streamer::RxStreamer;
use crate::types::{TuneRequest, TuneResult};
use crate::types::usrp_info::Info;
use crate::usrp::{StreamArgs, StreamCmd};

#[link(name = "uhd")]
extern {

	// uhd_error uhd_usrp_set_rx_subdev_spec(uhd_usrp_handle h, uhd_subdev_spec_handle subdev_spec, size_t mboard)
	// uhd_error uhd_usrp_get_rx_subdev_spec(uhd_usrp_handle h, size_t mboard, uhd_subdev_spec_handle subdev_spec_out)
	// uhd_error uhd_usrp_get_rx_subdev_name(uhd_usrp_handle h, size_t chan, char* rx_subdev_name_out, size_t strbuffer_len)
	// uhd_error uhd_usrp_get_rx_freq_range(uhd_usrp_handle h, size_t chan, uhd_meta_range_handle freq_range_out)
	// uhd_error uhd_usrp_get_fe_rx_freq_range(uhd_usrp_handle h, size_t chan, uhd_meta_range_handle freq_range_out)
	// uhd_error uhd_usrp_get_rx_lo_names(uhd_usrp_handle h, size_t chan, uhd_string_vector_handle *rx_lo_names_out)
	// uhd_error uhd_usrp_set_rx_lo_source(uhd_usrp_handle h, const char* src, const char* name, size_t chan)
	// uhd_error uhd_usrp_get_rx_lo_source(uhd_usrp_handle h, const char* name, size_t chan, char* rx_lo_source_out, size_t strbuffer_len)
	// uhd_error uhd_usrp_get_rx_lo_sources(uhd_usrp_handle h, const char* name, size_t chan, uhd_string_vector_handle *rx_lo_sources_out)
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
	// uhd_error uhd_usrp_get_rx_antennas(uhd_usrp_handle h, size_t chan, uhd_string_vector_handle *antennas_out)
	// uhd_error uhd_usrp_get_rx_sensor_names(uhd_usrp_handle h, size_t chan, uhd_string_vector_handle *sensor_names_out)
	// uhd_error uhd_usrp_get_rx_bandwidth_range(uhd_usrp_handle h, size_t chan, uhd_meta_range_handle bandwidth_range_out)
	// uhd_error uhd_usrp_get_rx_sensor(uhd_usrp_handle h, const char* name, size_t chan, uhd_sensor_value_handle *sensor_value_out)
	// uhd_error uhd_usrp_set_rx_dc_offset_enabled(uhd_usrp_handle h, bool enb, size_t chan)
	// uhd_error uhd_usrp_set_rx_iq_balance_enabled(uhd_usrp_handle h, bool enb, size_t chan)

	fn uhd_usrp_get_rx_info(h:usize, chan:size_t, info_out:&mut Info) -> isize;
	fn uhd_usrp_get_rx_num_channels(h:usize, num_channels_out:&mut size_t) -> isize;

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

impl std::io::Read for super::USRP {

	fn read(&mut self, buff: &mut [u8]) -> std::result::Result<usize, std::io::Error> { 

		let rx_streamer:&mut RxStreamer = self.opt_rx_streamer.as_mut().unwrap();

		rx_streamer.read(buff)
	}

}

impl super::USRP {

	pub fn get_rx_bandwidth(&self, chan:usize) -> Result<f64, &'static str> {
		let mut ans:f64 = 0.0;
		match unsafe { uhd_usrp_get_rx_bandwidth(self.handle, chan, &mut ans) } {
			0 => Ok(ans),
			_ => Err("Unable to get RX bandwidth")
		}
	}

	pub fn set_rx_bandwidth(&mut self, bandwidth:f64, chan:usize) -> Result<(), &'static str> {
		if self.last_commanded_bw == Some(bandwidth) { 
			Ok(())
		} else {
			self.last_commanded_bw = Some(bandwidth);		
			match unsafe { uhd_usrp_set_rx_bandwidth(self.handle, bandwidth, chan) } {
				0 => Ok(()),
				_ => Err("Unable to set RX bandwidth")
			}
		}
	}

	pub fn start_continuous_stream<W: Any, U: Any>(&mut self, args:&str) -> Result<(), &'static str> {
		if self.opt_rx_streamer.is_none() { self.get_rx_stream::<W, U>(args)?; }

		let rx_streamer:&mut RxStreamer = self.opt_rx_streamer.as_mut().unwrap();

		let stream_cmd_start = StreamCmd::start_continuous_now();
		rx_streamer.stream(&stream_cmd_start)?;

		Ok(())
	}

	pub fn stop_continuous_stream(&mut self) -> Result<(), &'static str> {

		let rx_streamer:&mut RxStreamer = self.opt_rx_streamer.as_mut().unwrap();

		let stream_cmd_stop  = StreamCmd::stop_continuous_now();
		rx_streamer.stream(&stream_cmd_stop)?;

		Ok(())
	}

	fn get_rx_stream<W: Any, U: Any>(&mut self, args:&str) -> Result<(), &'static str> {
		// Note: This implementation assumes that you always want to create a new RxStreamer for every stream you want
		// to create.  If you're going to be creating and destroying streams all the time, it might be more efficient to
		// reuse instances of an RxStreamer.  If that ends up being the case, we could potentially create some kind of 
		// pool of them inside the USRP struct and still provide the same abstraction to the outside
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

		let mut rx_streamer = RxStreamer::new(stream_args.n_channels as usize)?;
		if unsafe { uhd_usrp_get_rx_stream(self.handle, &stream_args, rx_streamer.get_handle()) } != 0 {
			return Err("Unable to get an RxStream");
		}

		rx_streamer.get_max_num_samps()?;

		self.opt_rx_streamer = Some(rx_streamer);

		Ok(())
	}

	pub fn rx_num_channels(&self) -> Result<usize, &'static str> {
		let mut ans:usize = 0;
		match unsafe { uhd_usrp_get_rx_num_channels(self.handle, &mut ans) } {
			0 => Ok(ans),
			_ => Err("Unable to get number of RX channels")
		}
	}

	pub fn get_rx_info(&self, chan:usize) -> Result<Info, &'static str> {
		let mut ans = Info::null();
		match unsafe { uhd_usrp_get_rx_info(self.handle, chan, &mut ans) } {
			0 => Ok(ans),
			_ => Err("Unable to get RX info")
		}
	}

	pub fn set_rx_rate(&mut self, rate:f64, chan:usize) -> Result<(), &'static str> {
		if self.last_commanded_rate == Some(rate) { 
			Ok(())
		} else {
			self.last_commanded_rate = Some(rate);
			match unsafe { uhd_usrp_set_rx_rate(self.handle, rate, chan) } {
				0 => Ok(()),
				_ => Err("Unable to set RX rate")
			}			
		}
	}

	pub fn get_rx_rate(&self, chan:usize) -> Result<f64, &'static str> {
		let mut ans:f64 = 0.0;
		match unsafe { uhd_usrp_get_rx_rate(self.handle, chan, &mut ans) } {
			0 => Ok(ans),
			_ => Err("Unable to get RX rate")
		}
	}

	pub fn set_rx_gain(&mut self, gain:f64, chan:usize, gain_name:&str) -> Result<(), &'static str> {
		if self.last_commanded_gain == Some(gain) { 
			Ok(()) 
		} else {
			self.last_commanded_gain = Some(gain);
			let gain_name_c:CString = CString::new(gain_name).unwrap();
			match unsafe { uhd_usrp_set_rx_gain(self.handle, gain, chan, gain_name_c.as_ptr()) } {
				0 => Ok(()),
				_ => Err("Unable to set RX gain")
			}
		}
	}

	pub fn get_rx_gain(&self, chan:usize, gain_name:&str) -> Result<f64, &'static str> {
		let gain_name_c:CString = CString::new(gain_name).unwrap();
		let mut gain_out:f64 = 0.0;
		match unsafe { uhd_usrp_get_rx_gain(self.handle, chan, gain_name_c.as_ptr(), &mut gain_out) } {
			0 => Ok(gain_out),
			_ => Err("Unable to get RX gain")
		}
	}

	pub fn set_rx_freq(&mut self, tune_request:&TuneRequest, chan:usize) -> Result<TuneResult, &'static str> {
		let mut tune_result:TuneResult = TuneResult::default();
		match unsafe { uhd_usrp_set_rx_freq(self.handle, tune_request, chan, &mut tune_result) } {
			0 => Ok(tune_result),
			_ => Err("Unable to set RX freq")
		}
	}

	pub fn get_rx_freq(&self, chan:usize) -> Result<f64, &'static str> {
		let mut freq_out:f64 = 0.0;
		match unsafe { uhd_usrp_get_rx_freq(self.handle, chan, &mut freq_out) } {
			0 => Ok(freq_out),
			_ => Err("Unable to get RX freq")
		}
	}


}