
use std::ffi::CString;

use libc::{size_t, c_char};

use crate::usrp::USRP;
use crate::types::string_vector::StringVector;
use crate::{check_err, UhdError};

#[link(name = "uhd")]
extern {

    fn uhd_usrp_get_time_now(h:usize, mboard:size_t, full_secs_out:&mut i64, frac_secs_out:&mut f64) -> UhdError;
    fn uhd_usrp_get_time_last_pps(h:usize, mboard:usize, full_secs_out:&mut i64, frac_secs_out:&mut f64) -> UhdError;

    // uhd_error uhd_usrp_set_time_now(uhd_usrp_handle h, int64_t full_secs, double frac_secs, size_t mboard)
    fn uhd_usrp_set_time_next_pps(h:usize, full_secs:i64, frac_secs:f64, mboard:usize) -> UhdError;
    fn uhd_usrp_set_time_unknown_pps(h:usize, full_secs:i64, frac_secs:f64) -> UhdError;

    // uhd_error uhd_usrp_get_time_synchronized(uhd_usrp_handle h, bool *result_out)
    
    fn uhd_usrp_set_command_time(h:usize, full_secs:i64, frac_secs:f64, mboard:size_t) -> UhdError;
    fn uhd_usrp_clear_command_time(h:usize, mboard:size_t) -> UhdError;

    fn uhd_usrp_set_time_source(h:usize, time_source:*const c_char, mboard:size_t) -> isize;
    fn uhd_usrp_get_time_source(h:usize, mboard:size_t, time_source_out:*const c_char, strbuffer_len:size_t) -> UhdError;
    fn uhd_usrp_get_time_sources(h:usize, mboard:size_t, time_sources_out:&mut usize) -> isize;

    fn uhd_usrp_set_clock_source(h:usize, clock_source:*const c_char, mboard:size_t) -> isize;
    fn uhd_usrp_get_clock_source(h:usize, mboard:size_t, clock_source_out:*const c_char, strbuffer_len:size_t) -> UhdError;
    fn uhd_usrp_get_clock_sources(h:usize, mboard:size_t, clock_sources_out:&mut usize) -> isize;

    fn uhd_usrp_set_clock_source_out(h:usize, enb:bool, mboard:size_t) -> isize;
    fn uhd_usrp_set_time_source_out(h:usize, enb:bool, mboard:size_t) -> isize;

}

impl USRP {

    pub fn set_time_unknown_pps(&mut self, full_secs:i64, frac_secs:f64, mboard:usize) -> Result<(), &'static str> {
        match unsafe { uhd_usrp_set_time_unknown_pps(self.handle, full_secs, frac_secs, mboard) } {
            0 => Ok(()),
            _ => Err("Unable to set unknown PPS time"),
        }
    }

    pub fn set_command_time(&mut self, full_secs:i64, frac_secs:f64, mboard:usize) -> Result<(), &'static str> {
        match unsafe { uhd_usrp_set_command_time(self.handle, full_secs, frac_secs, mboard) } {
            0 => Ok(()),
            _ => Err("Unable to set command time"),
        }
    }

    pub fn clear_command_time(&mut self, mboard:usize) -> Result<(), &'static str> {
        match unsafe { uhd_usrp_clear_command_time(self.handle, mboard) } {
            0 => Ok(()),
            _ => Err("Unable to clear command time"),
        }
    }

    pub fn set_time_next_pps(&mut self, full_secs:i64, frac_secs:f64, mboard:usize) -> Result<(), &'static str> {
        match unsafe { uhd_usrp_set_time_next_pps(self.handle, full_secs, frac_secs, mboard) } {
            0 => Ok(()),
            _ => Err("Unable to set next PPS time"),
        }
    }

    pub fn get_time_now(&self, mboard:usize) -> Result<(i64, f64), &'static str> {
        let mut full_secs_out:i64 = 0;
        let mut frac_secs_out:f64 = 0.0;
        let result = unsafe{ uhd_usrp_get_time_now(self.handle, mboard, &mut full_secs_out, &mut frac_secs_out) };
        check_err((full_secs_out, frac_secs_out), result)
    }

    pub fn get_time_last_pps(&self, mboard:usize) -> Result<(i64, f64), &'static str> {
        let mut full_secs_out:i64 = 0;
        let mut frac_secs_out:f64 = 0.0;
        let result = unsafe{ uhd_usrp_get_time_last_pps(self.handle, mboard, &mut full_secs_out, &mut frac_secs_out) };
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

    pub fn set_time_source(&mut self, time_source:&str, mboard:usize) -> Result<(), &'static str> {
        let time_source_c:CString = CString::new(time_source).unwrap();
        match unsafe { uhd_usrp_set_time_source(self.handle, time_source_c.as_ptr(), mboard) } {
            0 => Ok(()),
            _ => Err("Unable to set time source")
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
