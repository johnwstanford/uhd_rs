
use libc::{size_t, c_char};
use crate::c_interop::collect_cstr;

use crate::check_err;

pub mod subdev_spec;

mod impl_sensors;
mod impl_static;
mod impl_time;
mod impl_rx;
mod impl_tx;

#[link(name = "uhd")]
extern {

	fn uhd_usrp_last_error(h: usize, error_out: *mut u8, strbuffer_len: size_t) -> isize;

	// uhd_error uhd_usrp_set_master_clock_rate(uhd_usrp_handle h, double rate, size_t mboard)
	// uhd_error uhd_usrp_get_master_clock_rate(uhd_usrp_handle h, size_t mboard, double *clock_rate_out)
	// uhd_error uhd_usrp_get_pp_string(uhd_usrp_handle h, char* pp_string_out, size_t strbuffer_len)
	// uhd_error uhd_usrp_get_mboard_name(uhd_usrp_handle h, size_t mboard, char* mboard_name_out, size_t strbuffer_len)

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
	
	fn uhd_usrp_free(uhd_usrp_handle: &mut usize);
}

pub struct USRP {
	handle: usize,
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

impl USRP {

	pub fn num_mboards(&self) -> Result<usize, &'static str> {
		let mut ans = 0;
		let result = unsafe{ uhd_usrp_get_num_mboards(self.handle, &mut ans) };
		check_err(ans, result)
	}

	pub fn last_error(&self) -> Result<String, &'static str> {
		let mut buff: Vec<u8> = vec![0; 256];
		unsafe {
			check_err((), uhd_usrp_last_error(self.handle, buff.as_mut_ptr(), buff.len()))?;
			Ok(collect_cstr(buff.as_ptr()))
		}
	}

}

impl std::ops::Drop for USRP {

	fn drop(&mut self) { 
		// TODO: consider checking the return value; right now we're not
		unsafe { uhd_usrp_free(&mut self.handle); } 
	}

}