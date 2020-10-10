
use std::ffi::CString;
use std::any::{Any, TypeId};

use crate::ffi::types::{TuneRequest, TuneResult};
use crate::ffi::usrp::StreamArgs;

use crate::rx_streamer::RxStreamer;
use crate::tx_streamer::TxStreamer;
use crate::types::string_vector::StringVector;
use crate::types::usrp_info::Info;

#[derive(Debug)]
pub struct USRP {
	handle:usize
}

mod impl_static;
mod impl_rx;

impl USRP {
	
	pub fn get_tx_info(&self, chan:usize) -> Result<Info, &'static str> {
		let mut ans = Info::null();
		match unsafe { crate::ffi::usrp::uhd_usrp_get_tx_info(self.handle, chan, &mut ans) } {
			0 => Ok(ans),
			_ => Err("Unable to get Tx info")
		}
	}

	pub fn get_tx_antennas(&self, chan:usize) -> Result<Vec<String>, &'static str> {
		let mut string_vec = StringVector::new()?;
		match unsafe { crate::ffi::usrp::uhd_usrp_get_tx_antennas(self.handle, chan, &mut string_vec.handle) } {
			0 => Ok(string_vec.get_rust_vec()?),
			_ => Err("Unable to retrieve TX antennas")
		}
	}

	pub fn set_tx_rate(&mut self, rate:f64, chan:usize) -> Result<(), &'static str> {
		match unsafe { crate::ffi::usrp::uhd_usrp_set_tx_rate(self.handle, rate, chan) } {
			0 => Ok(()),
			_ => Err("Unable to set TX rate")
		}
	}

	pub fn get_tx_rate(&self, chan:usize) -> Result<f64, &'static str> {
		let mut ans:f64 = 0.0;
		match unsafe { crate::ffi::usrp::uhd_usrp_get_tx_rate(self.handle, chan, &mut ans) } {
			0 => Ok(ans),
			_ => Err("Unable to get TX rate")
		}
	}

	pub fn set_rx_gain(&mut self, gain:f64, chan:usize, gain_name:&str) -> Result<(), &'static str> {
		let gain_name_c:CString = CString::new(gain_name).unwrap();
		match unsafe { crate::ffi::usrp::uhd_usrp_set_rx_gain(self.handle, gain, chan, gain_name_c.as_ptr()) } {
			0 => Ok(()),
			_ => Err("Unable to set RX gain")
		}
	}

	pub fn set_tx_gain(&mut self, gain:f64, chan:usize, gain_name:&str) -> Result<(), &'static str> {
		let gain_name_c:CString = CString::new(gain_name).unwrap();
		match unsafe { crate::ffi::usrp::uhd_usrp_set_tx_gain(self.handle, gain, chan, gain_name_c.as_ptr()) } {
			0 => Ok(()),
			_ => Err("Unable to set TX gain")
		}
	}

	pub fn get_rx_gain(&self, chan:usize, gain_name:&str) -> Result<f64, &'static str> {
		let gain_name_c:CString = CString::new(gain_name).unwrap();
		let mut gain_out:f64 = 0.0;
		match unsafe { crate::ffi::usrp::uhd_usrp_get_rx_gain(self.handle, chan, gain_name_c.as_ptr(), &mut gain_out) } {
			0 => Ok(gain_out),
			_ => Err("Unable to get RX gain")
		}
	}

	pub fn get_tx_gain(&self, chan:usize, gain_name:&str) -> Result<f64, &'static str> {
		let gain_name_c:CString = CString::new(gain_name).unwrap();
		let mut gain_out:f64 = 0.0;
		match unsafe { crate::ffi::usrp::uhd_usrp_get_tx_gain(self.handle, chan, gain_name_c.as_ptr(), &mut gain_out) } {
			0 => Ok(gain_out),
			_ => Err("Unable to get TX gain")
		}
	}

	pub fn set_rx_freq(&mut self, tune_request:&TuneRequest, chan:usize) -> Result<TuneResult, &'static str> {
		let mut tune_result:TuneResult = TuneResult::default();
		match unsafe { crate::ffi::usrp::uhd_usrp_set_rx_freq(self.handle, tune_request, chan, &mut tune_result) } {
			0 => Ok(tune_result),
			_ => Err("Unable to set RX freq")
		}
	}

	pub fn set_tx_freq(&mut self, tune_request:&TuneRequest, chan:usize) -> Result<TuneResult, &'static str> {
		let mut tune_result:TuneResult = TuneResult::default();
		match unsafe { crate::ffi::usrp::uhd_usrp_set_tx_freq(self.handle, tune_request, chan, &mut tune_result) } {
			0 => Ok(tune_result),
			_ => Err("Unable to set TX freq")
		}
	}

	pub fn get_rx_freq(&self, chan:usize) -> Result<f64, &'static str> {
		let mut freq_out:f64 = 0.0;
		match unsafe { crate::ffi::usrp::uhd_usrp_get_rx_freq(self.handle, chan, &mut freq_out) } {
			0 => Ok(freq_out),
			_ => Err("Unable to get RX freq")
		}
	}

	pub fn get_tx_freq(&self, chan:usize) -> Result<f64, &'static str> {
		let mut freq_out:f64 = 0.0;
		match unsafe { crate::ffi::usrp::uhd_usrp_get_tx_freq(self.handle, chan, &mut freq_out) } {
			0 => Ok(freq_out),
			_ => Err("Unable to get TX freq")
		}
	}

	pub fn get_rx_stream<W: Any, U: Any>(&mut self, args:&str) -> Result<RxStreamer, &'static str> {
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
		if unsafe { crate::ffi::usrp::uhd_usrp_get_rx_stream(self.handle, &stream_args, rx_streamer.get_handle()) } != 0 {
			return Err("Unable to get an RxStream");
		}

		rx_streamer.get_max_num_samps()?;

		Ok(rx_streamer)
	}

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
		if unsafe { crate::ffi::usrp::uhd_usrp_get_tx_stream(self.handle, &stream_args, tx_streamer.get_handle()) } != 0 {
			return Err("Unable to get an TxStream");
		}

		tx_streamer.get_max_num_samps()?;

		Ok(tx_streamer)
	}

}

impl std::ops::Drop for USRP {

	fn drop(&mut self) { 
		// TODO: consider checking the return value; right now we're not
		unsafe { crate::ffi::usrp::uhd_usrp_free(&mut self.handle); } 
	}

}