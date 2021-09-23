use libc::{size_t, c_char};

use crate::usrp::USRP;
use crate::types::string_vector::StringVector;

#[link(name = "uhd")]
extern {

    // uhd_error uhd_usrp_get_mboard_sensor(uhd_usrp_handle h, const char* name, size_t mboard, uhd_sensor_value_handle *sensor_value_out)
    fn uhd_usrp_get_mboard_sensor_names(h:usize, mboard:size_t, mboard_sensor_names_out:&mut usize) -> isize;


}

impl USRP {

    pub fn get_sensor_names(&self, mboard:usize) -> Result<Vec<String>, &'static str> {
        let mut string_vec = StringVector::new()?;
        match unsafe { uhd_usrp_get_mboard_sensor_names(self.handle, mboard, &mut string_vec.handle) } {
            0 => Ok(string_vec.get_rust_vec()?),
            _ => Err("Unable to get time sources")
        }
    }



}