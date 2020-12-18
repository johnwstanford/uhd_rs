
use std::ffi::CString;

use libc::{size_t, c_char};

use crate::check_err;
use crate::types::string_vector::StringVector;

/*
	Usage of time_spec_t:
	- In StreamCmd to tell the streamer when to stream if not now (relative to the time the streamer was created?)
	- In RxMetadata to give the time of the first sample (relative to the time the streamer was created?)
	- In TxMetadata to give the time of the first sample (relative to the time the streamer was created?)
	- In AsyncMetadata (not implemented in Rust yet)
	- uhd::rfnoc::set_time_now
	- uhd::rfnox::set_time_next_pps
	- uhd::rfnoc::get_time_now
	- uhd::rfnoc::get_time_last_pps
	- FPGA control and communication (see block_ctrl_base.hpp)
	- Generic daughterboard interface (see dboard_iface.hpp)
	- uhd::usrp::multi_usrp::get_time_now gives "current USRP time"
	- uhd::usrp::multi_usrp::get_time_last_pps
	- uhd::usrp::multi_usrp::set_time_now
	- uhd::usrp::multi_usrp::set_time_next_pps
	- uhd::usrp::multi_usrp::set_time_unknown_pps
	- uhd::usrp::multi_usrp::set_command_time
*/

#[link(name = "uhd")]
extern {

	// uhd_error uhd_usrp_last_error(uhd_usrp_handle h, char* error_out, size_t strbuffer_len)
	// uhd_error uhd_usrp_set_master_clock_rate(uhd_usrp_handle h, double rate, size_t mboard)
	// uhd_error uhd_usrp_get_master_clock_rate(uhd_usrp_handle h, size_t mboard, double *clock_rate_out)
	// uhd_error uhd_usrp_get_pp_string(uhd_usrp_handle h, char* pp_string_out, size_t strbuffer_len)
	// uhd_error uhd_usrp_get_mboard_name(uhd_usrp_handle h, size_t mboard, char* mboard_name_out, size_t strbuffer_len)

	// uhd_error uhd_usrp_get_time_now(uhd_usrp_handle h, size_t mboard, int64_t *full_secs_out, double *frac_secs_out)
	fn uhd_usrp_get_time_now(h:usize, mboard:size_t, full_secs_out:&mut i64, frac_secs_out:&mut f64) -> isize;

	// uhd_error uhd_usrp_get_time_last_pps(uhd_usrp_handle h, size_t mboard, int64_t *full_secs_out, double *frac_secs_out)
	// uhd_error uhd_usrp_set_time_now(uhd_usrp_handle h, int64_t full_secs, double frac_secs, size_t mboard)
	// uhd_error uhd_usrp_set_time_next_pps(uhd_usrp_handle h, int64_t full_secs, double frac_secs, size_t mboard)
	// uhd_error uhd_usrp_set_time_unknown_pps(uhd_usrp_handle h, int64_t full_secs, double frac_secs)
	// uhd_error uhd_usrp_get_time_synchronized(uhd_usrp_handle h, bool *result_out)
	// uhd_error uhd_usrp_set_command_time(uhd_usrp_handle h, int64_t full_secs, double frac_secs, size_t mboard)
	// uhd_error uhd_usrp_clear_command_time(uhd_usrp_handle h, size_t mboard)
	// uhd_error uhd_usrp_get_mboard_sensor(uhd_usrp_handle h, const char* name, size_t mboard, uhd_sensor_value_handle *sensor_value_out)
	// uhd_error uhd_usrp_get_mboard_sensor_names(uhd_usrp_handle h, size_t mboard, uhd_string_vector_handle *mboard_sensor_names_out)
	// uhd_error uhd_usrp_set_user_register(uhd_usrp_handle h, uint8_t addr, uint32_t data, size_t mboard)
	// uhd_error uhd_usrp_get_mboard_eeprom(uhd_usrp_handle h, uhd_mboard_eeprom_handle mb_eeprom, size_t mboard)
	// uhd_error uhd_usrp_set_mboard_eeprom(uhd_usrp_handle h, uhd_mboard_eeprom_handle mb_eeprom, size_t mboard)
	// uhd_error uhd_usrp_get_dboard_eeprom(uhd_usrp_handle h, uhd_dboard_eeprom_handle db_eeprom, const char* unit, const char* slot, size_t mboard)
	// uhd_error uhd_usrp_set_dboard_eeprom(uhd_usrp_handle h, uhd_dboard_eeprom_handle db_eeprom, const char* unit, const char* slot, size_t mboard)
		
	// uhd_error uhd_usrp_get_gpio_banks(uhd_usrp_handle h, size_t mboard, uhd_string_vector_handle *gpio_banks_out)
	// uhd_error uhd_usrp_set_gpio_attr(uhd_usrp_handle h, const char* bank, const char* attr, uint32_t value, uint32_t mask, size_t mboard)
	// uhd_error uhd_usrp_get_gpio_attr(uhd_usrp_handle h, const char* bank, const char* attr, size_t mboard, uint32_t *attr_out)
	// uhd_error uhd_usrp_enumerate_registers(uhd_usrp_handle h, size_t mboard, uhd_string_vector_handle *registers_out)
	// uhd_error uhd_usrp_get_register_info(uhd_usrp_handle h, const char* path, size_t mboard, uhd_usrp_register_info_t *register_info_out)
	// uhd_error uhd_usrp_write_register(uhd_usrp_handle h, const char* path, uint32_t field, uint64_t value, size_t mboard)
	// uhd_error uhd_usrp_read_register(uhd_usrp_handle h, const char* path, uint32_t field, size_t mboard, uint64_t *value_out)

	fn uhd_usrp_get_num_mboards(h:usize, num_mboards_out:&mut size_t) -> isize;
	
	// uhd_error uhd_usrp_set_time_source(uhd_usrp_handle h, const char* time_source, size_t mboard)
	fn uhd_usrp_get_time_source(h:usize, mboard:size_t, time_source_out:*const c_char, strbuffer_len:size_t) -> isize;
	fn uhd_usrp_get_time_sources(h:usize, mboard:size_t, time_sources_out:&mut usize) -> isize;
	
	fn uhd_usrp_set_clock_source(h:usize, clock_source:*const c_char, mboard:size_t) -> isize;
	fn uhd_usrp_get_clock_source(h:usize, mboard:size_t, clock_source_out:*const c_char, strbuffer_len:size_t) -> isize;
	fn uhd_usrp_get_clock_sources(h:usize, mboard:size_t, clock_sources_out:&mut usize) -> isize;
	
	fn uhd_usrp_set_clock_source_out(h:usize, enb:bool, mboard:size_t) -> isize;
	fn uhd_usrp_set_time_source_out(h:usize, enb:bool, mboard:size_t) -> isize;

	fn uhd_usrp_free(uhd_usrp_handle: &mut usize);	
}

pub struct USRP {
	handle:usize,
	last_commanded_rate:Option<f64>,
	last_commanded_gain:Option<f64>,
	last_commanded_bw:Option<f64>,
}

#[repr(C)]
pub struct StreamArgs {
    pub cpu_format:*const c_char,	// Format of host memory
    pub otw_format:*const c_char,	// Over-the-wire format		
    pub args:*const c_char,			// Other stream args
    pub channel_list:*const size_t, // Array that lists channels
    pub n_channels:isize			// Number of channels
}

#[repr(C)]
pub enum StreamMode {
	StartContinuous =  97,  
	StopContinuous  = 111,
	NumSampsAndDone = 100, 
	NumSampsAndMore = 109
}

#[repr(C)]
pub struct StreamCmd {
    pub stream_mode:StreamMode,		// How streaming is issued to the device
    pub num_samps: size_t,			// Number of samples
    pub stream_now:bool,			// Stream now?
    pub time_spec_full_secs:i64,	// If not now, then full seconds into future to stream
    pub time_spec_frac_secs:f64		// If not now, then fractional seconds into future to stream
}

impl StreamCmd {

	pub fn start_continuous_now() -> Self { Self {
	    stream_mode:StreamMode::StartContinuous, num_samps: 0,
	    stream_now: true, time_spec_full_secs: 0, time_spec_frac_secs: 0.0
	}}
	
	pub fn stop_continuous_now() -> Self { Self {
	    stream_mode:StreamMode::StopContinuous, num_samps: 0,
	    stream_now: true, time_spec_full_secs: 0, time_spec_frac_secs: 0.0
	}}
	
}

mod impl_static;
mod impl_rx;
mod impl_tx;

impl USRP {

	pub fn num_mboards(&self) -> Result<usize, &'static str> {
		let mut ans = 0;
		let result = unsafe{ uhd_usrp_get_num_mboards(self.handle, &mut ans) };
		check_err(ans, result)
	}

	pub fn get_time_now(&self, mboard:usize) -> Result<(i64, f64), &'static str> {
		let mut full_secs_out:i64 = 0;
		let mut frac_secs_out:f64 = 0.0;
		let result = unsafe{ uhd_usrp_get_time_now(self.handle, mboard, &mut full_secs_out, &mut frac_secs_out) };
		check_err((full_secs_out, frac_secs_out), result)
	}

	pub fn get_time_source(&self, mboard:usize) -> Result<String, &'static str> {
		let buffer_init = "                                        ";
		let cstr_ans:CString = CString::new(buffer_init).map_err(|_| "Unable to create CString")?;
        match unsafe { uhd_usrp_get_time_source(self.handle, mboard, cstr_ans.as_ptr(), buffer_init.len()) } {
            0 => {
            	let ans:String = cstr_ans.into_string().map_err(|_| "Unable to convert CString to String")?;
				let ans:String = ans.trim_matches(char::from(0)).to_owned();
            	Ok(ans)
            },
            _ => Err("Unable to get time source")
        }

	}

	pub fn get_time_sources(&self, mboard:usize) -> Result<Vec<String>, &'static str> {
		let mut string_vec = StringVector::new()?;
		match unsafe { uhd_usrp_get_time_sources(self.handle, mboard, &mut string_vec.handle) } {
			0 => Ok(string_vec.get_rust_vec()?),
			_ => Err("Unable to get time sources")
		}
	}

	pub fn get_clock_source(&self, mboard:usize) -> Result<String, &'static str> {
		let buffer_init = "                                        ";
		let cstr_ans:CString = CString::new(buffer_init).map_err(|_| "Unable to create CString")?;
        match unsafe { uhd_usrp_get_clock_source(self.handle, mboard, cstr_ans.as_ptr(), buffer_init.len()) } {
            0 => {
            	let ans:String = cstr_ans.into_string().map_err(|_| "Unable to convert CString to String")?;
				let ans:String = ans.trim_matches(char::from(0)).to_owned();
            	Ok(ans)
            },
            _ => Err("Unable to get clock source")
        }

	}

	pub fn set_clock_source(&mut self, clock_source:&str, mboard:usize) -> Result<(), &'static str> {
		let clock_source_c:CString = CString::new(clock_source).unwrap();
		match unsafe { uhd_usrp_set_clock_source(self.handle, clock_source_c.as_ptr(), mboard) } {
			0 => Ok(()),
			_ => Err("Unable to set clock source")
		}
	}

	pub fn get_clock_sources(&self, mboard:usize) -> Result<Vec<String>, &'static str> {
		let mut string_vec = StringVector::new()?;
		match unsafe { uhd_usrp_get_clock_sources(self.handle, mboard, &mut string_vec.handle) } {
			0 => Ok(string_vec.get_rust_vec()?),
			_ => Err("Unable to get clock sources")
		}
	}

	pub fn set_clock_source_out(&mut self, mboard:usize, enb:bool) -> Result<(), &'static str> {
		match unsafe { uhd_usrp_set_clock_source_out(self.handle, enb, mboard) } {
			0 => Ok(()),
			_ => Err("Unable to set clock source out")
		}
	}

	pub fn set_time_source_out(&mut self, mboard:usize, enb:bool) -> Result<(), &'static str> {
		match unsafe { uhd_usrp_set_time_source_out(self.handle, enb, mboard) } {
			0 => Ok(()),
			_ => Err("Unable to set clock source out")
		}
	}

}

impl std::ops::Drop for USRP {

	fn drop(&mut self) { 
		// TODO: consider checking the return value; right now we're not
		unsafe { uhd_usrp_free(&mut self.handle); } 
	}

}