
use libc::c_char;

use std::ffi::CString;

use crate::UhdError;

// From uhd/types/sensors.h

#[cfg(test)]
mod tests;

pub type SensorValueHandle = usize;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DataType {
    Boolean = 98,
    Integer = 105,
    RealNum = 114,
    String  = 115
}

#[link(name = "uhd")]
extern {

    fn uhd_sensor_value_make(h:*mut SensorValueHandle) -> UhdError;
    fn uhd_sensor_value_make_from_bool(h:*mut SensorValueHandle, name:*const c_char, value:bool, utrue:*const c_char, ufalse:*const c_char) -> UhdError;
    // uhd_error uhd_sensor_value_make_from_int(uhd_sensor_value_handle* h, const char* name, int value, const char* unit, const char* formatter);
    // uhd_error uhd_sensor_value_make_from_realnum(uhd_sensor_value_handle* h, const char* name, double value, const char* unit, const char* formatter);
    // uhd_error uhd_sensor_value_make_from_string(uhd_sensor_value_handle* h, const char* name, const char* value, const char* unit);

    fn uhd_sensor_value_free(h:*mut SensorValueHandle) -> UhdError;

    // uhd_error uhd_sensor_value_to_bool(uhd_sensor_value_handle h, bool *value_out);
    // uhd_error uhd_sensor_value_to_int(uhd_sensor_value_handle h, int *value_out);
    // uhd_error uhd_sensor_value_to_realnum(uhd_sensor_value_handle h, double *value_out);
    fn uhd_sensor_value_name(h:SensorValueHandle, name_out:*mut u8, strbuffer_len:usize) -> UhdError;
    // uhd_error uhd_sensor_value_value(uhd_sensor_value_handle h, char* value_out, size_t strbuffer_len);
    // uhd_error uhd_sensor_value_unit(uhd_sensor_value_handle h, char* unit_out, size_t strbuffer_len);
    // uhd_error uhd_sensor_value_data_type(uhd_sensor_value_handle h, uhd_sensor_value_data_type_t *data_type_out);

    // uhd_error uhd_sensor_value_to_pp_string(uhd_sensor_value_handle h, char* pp_string_out, size_t strbuffer_len);
    // uhd_error uhd_sensor_value_last_error(uhd_sensor_value_handle h, char* error_out, size_t strbuffer_len);

}

pub struct SensorValue {
    handle: SensorValueHandle
}

impl SensorValue {

    pub fn new() -> Result<Self, &'static str> {
        let mut handle = SensorValueHandle::default();
        match unsafe { uhd_sensor_value_make(&mut handle) } {
            0 => Ok(Self{ handle }),
            _ => Err("Failed to create SensorValue"),
        }
    }

    pub fn from_bool(name:&str, value:bool, repr_true:&str, repr_false:&str) -> Result<Self, &'static str> {
        let name_c = CString::new(name).map_err(|_| "Unable to represent `name` as a CString")?;
        let repr_true_c = CString::new(repr_true).map_err(|_| "Unable to represent `repr_true` as a CString")?;
        let repr_false_c = CString::new(repr_false).map_err(|_| "Unable to represent `repr_false` as a CString")?;
        let mut handle = SensorValueHandle::default();
        match unsafe { uhd_sensor_value_make_from_bool(&mut handle, name_c.as_ptr(), value, repr_true_c.as_ptr(), repr_false_c.as_ptr()) } {
            0 => Ok(Self{ handle }),
            _ => Err("Failed to create SensorValue"),
        }
    }

    pub fn get_name(&self) -> Result<String, &'static str> {
        let mut buff:Vec<u8> = vec![0u8; 64];
        match unsafe { uhd_sensor_value_name(self.handle, buff.as_mut_ptr(), 64) } {
            0 => {
                let nonzero:Vec<u8> = buff.iter().take_while(|x| **x != 0).map(|x| *x).collect();
                Ok(String::from_utf8(nonzero).map_err(|_| "Unable to convert string returned from UHD as UTF-8")?)
            },
            _ => Err("Nonzero return value from C API call in SensorValue::get_name")
        }
    }

}

impl std::ops::Drop for SensorValue {
    fn drop(&mut self) {
        unsafe{ uhd_sensor_value_free(&mut self.handle); }
    }
}