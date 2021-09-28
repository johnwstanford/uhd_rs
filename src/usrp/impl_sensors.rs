use std::ffi::CString;

use libc::{size_t, c_char};

use crate::UhdError;
use crate::usrp::USRP;
use crate::types::string_vector::StringVector;
use crate::types::sensors::{SensorValueHandle, SensorValue};

#[link(name = "uhd")]
extern {

    fn uhd_usrp_get_mboard_sensor(h:usize, name:*const c_char, mboard:usize, sensor_value_out:*mut SensorValueHandle) -> UhdError;
    fn uhd_usrp_get_mboard_sensor_names(h:usize, mboard:size_t, mboard_sensor_names_out:&mut usize) -> UhdError;


}

impl USRP {

    pub fn get_mboard_sensor(&self, name:&str, mboard:usize) -> Result<SensorValue, &'static str> {
        let mut ans = SensorValue::new()?;
        let name_c = CString::new(name).map_err(|_| "Unable to represent `name` as a CString")?;
        match unsafe { uhd_usrp_get_mboard_sensor(self.handle, name_c.as_ptr(), mboard, ans.as_mut_ptr()) } {
            0 => Ok(ans),
            _ => Err("Unable to get motherboard sensor"),
        }
    }

    pub fn get_mboard_sensor_names(&self, mboard:usize) -> Result<Vec<String>, &'static str> {
        let mut string_vec = StringVector::new()?;
        match unsafe { uhd_usrp_get_mboard_sensor_names(self.handle, mboard, &mut string_vec.handle) } {
            0 => Ok(string_vec.get_rust_vec()?),
            _ => Err("Unable to get time sources")
        }
    }



}