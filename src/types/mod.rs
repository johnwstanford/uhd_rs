
use libc::c_char;

pub mod metadata;
pub mod string_vector;
pub mod usrp_info;

#[repr(C)]
pub enum TuneRequestPolicy { None = 78, Auto = 65, Manual = 77 }

#[repr(C)]
pub struct TuneRequest {
    pub target_freq:f64,					// Target frequency for RF chain in Hz
    pub rf_freq_policy:TuneRequestPolicy, 	// RF frequency policy
    pub rf_freq:f64,						// RF frequency in Hz
    pub dsp_freq_policy:TuneRequestPolicy, 	// DSP frequency policy
    pub dsp_freq:f64,						// DSP frequency in Hz
    pub args:*const c_char					// Key-value pairs delimited by commas
}

#[repr(C)]
#[derive(Default, Debug)]
pub struct TuneResult {
    pub clipped_rf_freq:f64,	// Target RF frequency, clipped to be within system range
    pub target_rf_freq:f64,		// Target RF frequency, including RF FE offset
    pub actual_rf_freq:f64,		// Frequency to which RF LO is actually tuned
    pub target_dsp_freq:f64,	// Frequency the CORDIC must adjust the RF
    pub actual_dsp_freq:f64		// Frequency to which the CORDIC in the DSP actually tuned
}