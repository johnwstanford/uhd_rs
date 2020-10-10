
use libc::{c_char, size_t};

use crate::ffi::types::{TuneRequest, TuneResult};

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

#[link(name = "uhd")]
extern {

	// usrp.h:109-267
	pub fn uhd_rx_streamer_make(uhd_rx_streamer_handle: &mut usize) -> isize;
	pub fn uhd_rx_streamer_free(uhd_rx_streamer_handle: &mut usize) -> isize;

	// uhd_error uhd_rx_streamer_num_channels(uhd_rx_streamer_handle h, size_t *num_channels_out)

	pub fn uhd_rx_streamer_max_num_samps(h:usize, max_num_samps_out:&mut size_t) -> isize;
	pub fn uhd_rx_streamer_recv(h:usize, buffs:&*const u8, samps_per_buff:size_t, md:&usize, timeout:f64, one_packet:bool, items_recvd:&mut size_t) -> isize;
	pub fn uhd_rx_streamer_issue_stream_cmd(h:usize, stream_cmd:&StreamCmd) -> isize;
	pub fn uhd_rx_streamer_last_error(h:usize, error_out:*const c_char, strbuffer_len:size_t) -> isize;
	pub fn uhd_tx_streamer_make(h: &mut usize) -> isize;
	pub fn uhd_tx_streamer_free(h: &mut usize) -> isize;
	
	// uhd_error uhd_tx_streamer_num_channels(uhd_tx_streamer_handle h, size_t *num_channels_out)
	
	pub fn uhd_tx_streamer_max_num_samps(h:usize, max_num_samps_out:&mut size_t) -> isize;
	pub fn uhd_tx_streamer_send(h:usize, buffs:&*const u8, samps_per_buff:size_t, md:&usize, timeout:f64, items_sent:&mut size_t) -> isize;

	// uhd_error uhd_tx_streamer_recv_async_msg(uhd_tx_streamer_handle h, uhd_async_metadata_handle *md, double timeout, bool *valid)
	
	pub fn uhd_tx_streamer_last_error(h:usize, error_out:*const c_char, strbuffer_len:size_t) -> isize;

	// usrp.h:297-1265
	// uhd_error uhd_usrp_find(const char* args, uhd_string_vector_handle *strings_out)

	pub fn uhd_usrp_make(uhd_usrp_handle: &mut usize, args: *const c_char) -> isize;
	pub fn uhd_usrp_free(uhd_usrp_handle: &mut usize);

	// uhd_error uhd_usrp_last_error(uhd_usrp_handle h, char* error_out, size_t strbuffer_len)

	pub fn uhd_usrp_get_rx_stream(h:usize, stream_args:&StreamArgs, h_out:usize) -> isize;
	pub fn uhd_usrp_get_tx_stream(h:usize, stream_args:&StreamArgs, h_out:usize) -> isize;

	// uhd_error uhd_usrp_get_rx_info(uhd_usrp_handle h, size_t chan, uhd_usrp_rx_info_t *info_out)
	// uhd_error uhd_usrp_get_tx_info(uhd_usrp_handle h, size_t chan, uhd_usrp_tx_info_t *info_out)
	// uhd_error uhd_usrp_set_master_clock_rate(uhd_usrp_handle h, double rate, size_t mboard)
	// uhd_error uhd_usrp_get_master_clock_rate(uhd_usrp_handle h, size_t mboard, double *clock_rate_out)
	// uhd_error uhd_usrp_get_pp_string(uhd_usrp_handle h, char* pp_string_out, size_t strbuffer_len)
	// uhd_error uhd_usrp_get_mboard_name(uhd_usrp_handle h, size_t mboard, char* mboard_name_out, size_t strbuffer_len)
	// uhd_error uhd_usrp_get_time_now(uhd_usrp_handle h, size_t mboard, int64_t *full_secs_out, double *frac_secs_out)
	// uhd_error uhd_usrp_get_time_last_pps(uhd_usrp_handle h, size_t mboard, int64_t *full_secs_out, double *frac_secs_out)
	// uhd_error uhd_usrp_set_time_now(uhd_usrp_handle h, int64_t full_secs, double frac_secs, size_t mboard)
	// uhd_error uhd_usrp_set_time_next_pps(uhd_usrp_handle h, int64_t full_secs, double frac_secs, size_t mboard)
	// uhd_error uhd_usrp_set_time_unknown_pps(uhd_usrp_handle h, int64_t full_secs, double frac_secs)
	// uhd_error uhd_usrp_get_time_synchronized(uhd_usrp_handle h, bool *result_out)
	// uhd_error uhd_usrp_set_command_time(uhd_usrp_handle h, int64_t full_secs, double frac_secs, size_t mboard)
	// uhd_error uhd_usrp_clear_command_time(uhd_usrp_handle h, size_t mboard)
	// uhd_error uhd_usrp_set_time_source(uhd_usrp_handle h, const char* time_source, size_t mboard)
	// uhd_error uhd_usrp_get_time_source(uhd_usrp_handle h, size_t mboard, char* time_source_out, size_t strbuffer_len)
	// uhd_error uhd_usrp_get_time_sources(uhd_usrp_handle h, size_t mboard, uhd_string_vector_handle *time_sources_out)
	// uhd_error uhd_usrp_set_clock_source(uhd_usrp_handle h, const char* clock_source, size_t mboard)
	// uhd_error uhd_usrp_get_clock_source(uhd_usrp_handle h, size_t mboard, char* clock_source_out, size_t strbuffer_len)
	// uhd_error uhd_usrp_get_clock_sources(uhd_usrp_handle h, size_t mboard, uhd_string_vector_handle *clock_sources_out)
	// uhd_error uhd_usrp_set_clock_source_out(uhd_usrp_handle h, bool enb, size_t mboard)
	// uhd_error uhd_usrp_set_time_source_out(uhd_usrp_handle h, bool enb, size_t mboard)
	// uhd_error uhd_usrp_get_num_mboards(uhd_usrp_handle h, size_t *num_mboards_out)
	// uhd_error uhd_usrp_get_mboard_sensor(uhd_usrp_handle h, const char* name, size_t mboard, uhd_sensor_value_handle *sensor_value_out)
	// uhd_error uhd_usrp_get_mboard_sensor_names(uhd_usrp_handle h, size_t mboard, uhd_string_vector_handle *mboard_sensor_names_out)
	// uhd_error uhd_usrp_set_user_register(uhd_usrp_handle h, uint8_t addr, uint32_t data, size_t mboard)
	// uhd_error uhd_usrp_get_mboard_eeprom(uhd_usrp_handle h, uhd_mboard_eeprom_handle mb_eeprom, size_t mboard)
	// uhd_error uhd_usrp_set_mboard_eeprom(uhd_usrp_handle h, uhd_mboard_eeprom_handle mb_eeprom, size_t mboard)
	// uhd_error uhd_usrp_get_dboard_eeprom(uhd_usrp_handle h, uhd_dboard_eeprom_handle db_eeprom, const char* unit, const char* slot, size_t mboard)
	// uhd_error uhd_usrp_set_dboard_eeprom(uhd_usrp_handle h, uhd_dboard_eeprom_handle db_eeprom, const char* unit, const char* slot, size_t mboard)
	// uhd_error uhd_usrp_set_rx_subdev_spec(uhd_usrp_handle h, uhd_subdev_spec_handle subdev_spec, size_t mboard)
	// uhd_error uhd_usrp_get_rx_subdev_spec(uhd_usrp_handle h, size_t mboard, uhd_subdev_spec_handle subdev_spec_out)
	// uhd_error uhd_usrp_get_rx_num_channels(uhd_usrp_handle h, size_t *num_channels_out)
	// uhd_error uhd_usrp_get_rx_subdev_name(uhd_usrp_handle h, size_t chan, char* rx_subdev_name_out, size_t strbuffer_len)
	
	pub fn uhd_usrp_set_rx_rate(h:usize, rate:f64, chan:size_t) -> isize;
	pub fn uhd_usrp_get_rx_rate(h:usize, chan:size_t, rate_out:&mut f64) -> isize;
	pub fn uhd_usrp_set_rx_freq(h:usize, tune_request:&TuneRequest, chan:size_t, tune_result:&mut TuneResult) -> isize;
	pub fn uhd_usrp_get_rx_freq(h:usize, chan:size_t, freq_out:&mut f64) -> isize;

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
	
	pub fn uhd_usrp_set_rx_gain(h:usize, gain:f64, chan:size_t, gain_name:*const c_char) -> isize;

	// uhd_error uhd_usrp_set_normalized_rx_gain(uhd_usrp_handle h, double gain, size_t chan)
	// uhd_error uhd_usrp_set_rx_agc(uhd_usrp_handle h, bool enable, size_t chan)

	pub fn uhd_usrp_get_rx_gain(h:usize, chan:size_t, gain_name:*const c_char, gain_out:&mut f64) -> isize;

	// uhd_error uhd_usrp_get_normalized_rx_gain(uhd_usrp_handle h, size_t chan, double *gain_out)
	// uhd_error uhd_usrp_get_rx_gain_range(uhd_usrp_handle h, const char* name, size_t chan, uhd_meta_range_handle gain_range_out)
	// uhd_error uhd_usrp_get_rx_gain_names(uhd_usrp_handle h, size_t chan, uhd_string_vector_handle *gain_names_out)
	// uhd_error uhd_usrp_set_rx_antenna(uhd_usrp_handle h, const char* ant, size_t chan)
	// uhd_error uhd_usrp_get_rx_antenna(uhd_usrp_handle h, size_t chan, char* ant_out, size_t strbuffer_len)
	// uhd_error uhd_usrp_get_rx_antennas(uhd_usrp_handle h, size_t chan, uhd_string_vector_handle *antennas_out)
	// uhd_error uhd_usrp_get_rx_sensor_names(uhd_usrp_handle h, size_t chan, uhd_string_vector_handle *sensor_names_out)
	// uhd_error uhd_usrp_set_rx_bandwidth(uhd_usrp_handle h, double bandwidth, size_t chan)
	// uhd_error uhd_usrp_get_rx_bandwidth(uhd_usrp_handle h, size_t chan, double *bandwidth_out)
	// uhd_error uhd_usrp_get_rx_bandwidth_range(uhd_usrp_handle h, size_t chan, uhd_meta_range_handle bandwidth_range_out)
	// uhd_error uhd_usrp_get_rx_sensor(uhd_usrp_handle h, const char* name, size_t chan, uhd_sensor_value_handle *sensor_value_out)
	// uhd_error uhd_usrp_set_rx_dc_offset_enabled(uhd_usrp_handle h, bool enb, size_t chan)
	// uhd_error uhd_usrp_set_rx_iq_balance_enabled(uhd_usrp_handle h, bool enb, size_t chan)
	// uhd_error uhd_usrp_set_tx_subdev_spec(uhd_usrp_handle h, uhd_subdev_spec_handle subdev_spec, size_t mboard)
	// uhd_error uhd_usrp_get_tx_subdev_spec(uhd_usrp_handle h, size_t mboard, uhd_subdev_spec_handle subdev_spec_out)
	// uhd_error uhd_usrp_get_tx_num_channels(uhd_usrp_handle h, size_t *num_channels_out)
	// uhd_error uhd_usrp_get_tx_subdev_name(uhd_usrp_handle h, size_t chan, char* tx_subdev_name_out, size_t strbuffer_len)
	
	pub fn uhd_usrp_set_tx_rate(h:usize, rate:f64, chan:size_t) -> isize;
	pub fn uhd_usrp_get_tx_rate(h:usize, chan:size_t, rate_out:&mut f64) -> isize;

	// uhd_error uhd_usrp_get_tx_rates(uhd_usrp_handle h, size_t chan, uhd_meta_range_handle rates_out)
	
	pub fn uhd_usrp_set_tx_freq(h:usize, tune_request:&TuneRequest, chan:size_t, tune_result:&mut TuneResult) -> isize;
	pub fn uhd_usrp_get_tx_freq(h:usize, chan:size_t, freq_out:&mut f64) -> isize;
	
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
	
	pub fn uhd_usrp_set_tx_gain(h:usize, gain:f64, chan:size_t, gain_name:*const c_char) -> isize;
	
	// uhd_error uhd_usrp_set_normalized_tx_gain(uhd_usrp_handle h, double gain, size_t chan)
	// uhd_error uhd_usrp_get_tx_gain_range(uhd_usrp_handle h, const char* name, size_t chan, uhd_meta_range_handle gain_range_out)
	
	pub fn uhd_usrp_get_tx_gain(h:usize, chan:size_t, gain_name:*const c_char, gain_out:&mut f64) -> isize;
	
	// uhd_error uhd_usrp_get_normalized_tx_gain(uhd_usrp_handle h, size_t chan, double *gain_out)
	// uhd_error uhd_usrp_get_tx_gain_names(uhd_usrp_handle h, size_t chan, uhd_string_vector_handle *gain_names_out)
	// uhd_error uhd_usrp_set_tx_antenna(uhd_usrp_handle h, const char* ant, size_t chan)
	// uhd_error uhd_usrp_get_tx_antenna(uhd_usrp_handle h, size_t chan, char* ant_out, size_t strbuffer_len)
	
	// uhd_error uhd_usrp_get_tx_antennas(uhd_usrp_handle h, size_t chan, uhd_string_vector_handle *antennas_out)
	pub fn uhd_usrp_get_tx_antennas(h:usize, chan:size_t, antennas_out:&mut usize) -> isize;
	
	// uhd_error uhd_usrp_set_tx_bandwidth(uhd_usrp_handle h, double bandwidth, size_t chan)
	// uhd_error uhd_usrp_get_tx_bandwidth(uhd_usrp_handle h, size_t chan, double *bandwidth_out)
	// uhd_error uhd_usrp_get_tx_bandwidth_range(uhd_usrp_handle h, size_t chan, uhd_meta_range_handle bandwidth_range_out)
	// uhd_error uhd_usrp_get_tx_sensor(uhd_usrp_handle h, const char* name, size_t chan, uhd_sensor_value_handle *sensor_value_out)
	// uhd_error uhd_usrp_get_tx_sensor_names(uhd_usrp_handle h, size_t chan, uhd_string_vector_handle *sensor_names_out)
	// uhd_error uhd_usrp_get_gpio_banks(uhd_usrp_handle h, size_t mboard, uhd_string_vector_handle *gpio_banks_out)
	// uhd_error uhd_usrp_set_gpio_attr(uhd_usrp_handle h, const char* bank, const char* attr, uint32_t value, uint32_t mask, size_t mboard)
	// uhd_error uhd_usrp_get_gpio_attr(uhd_usrp_handle h, const char* bank, const char* attr, size_t mboard, uint32_t *attr_out)
	// uhd_error uhd_usrp_enumerate_registers(uhd_usrp_handle h, size_t mboard, uhd_string_vector_handle *registers_out)
	// uhd_error uhd_usrp_get_register_info(uhd_usrp_handle h, const char* path, size_t mboard, uhd_usrp_register_info_t *register_info_out)
	// uhd_error uhd_usrp_write_register(uhd_usrp_handle h, const char* path, uint32_t field, uint64_t value, size_t mboard)
	// uhd_error uhd_usrp_read_register(uhd_usrp_handle h, const char* path, uint32_t field, size_t mboard, uint64_t *value_out)

}

