
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
    fn uhd_sensor_value_make_from_int(h:*mut SensorValueHandle, name:*const c_char, value:i32, unit:*const c_char, formatter:*const c_char) -> UhdError;
    fn uhd_sensor_value_make_from_realnum(h:*mut SensorValueHandle, name:*const c_char, value:f64, unit:*const c_char, formatter:*const c_char) -> UhdError;
    fn uhd_sensor_value_make_from_string(h:*mut SensorValueHandle, name:*const c_char, value:*const c_char, unit:*const c_char) -> UhdError;

    fn uhd_sensor_value_free(h:*mut SensorValueHandle) -> UhdError;

    fn uhd_sensor_value_to_bool(h:SensorValueHandle, value_out:*mut bool) -> UhdError;
    fn uhd_sensor_value_to_int(h:SensorValueHandle, value_out:*mut i32) -> UhdError;
    fn uhd_sensor_value_to_realnum(h:SensorValueHandle, value_out:*mut f64) -> UhdError;
    fn uhd_sensor_value_name(h:SensorValueHandle, name_out:*mut u8, strbuffer_len:usize) -> UhdError;
    fn uhd_sensor_value_value(h:SensorValueHandle, value_out:*mut u8, strbuffer_len:usize) -> UhdError;
    fn uhd_sensor_value_unit(h:SensorValueHandle, unit_out:*mut u8, strbuffer_len:usize) -> UhdError;
    fn uhd_sensor_value_data_type(h:SensorValueHandle, data_type_out:*mut DataType) -> UhdError;

    fn uhd_sensor_value_to_pp_string(h:SensorValueHandle, pp_string_out:*mut u8, strbuffer_len:usize) -> UhdError;
    fn uhd_sensor_value_last_error(h:SensorValueHandle, error_out:*mut u8, strbuffer_len:usize) -> UhdError;

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

    pub fn from_string(name:&str, value:&str, unit:&str) -> Result<Self, &'static str> {
        let name_c = CString::new(name).map_err(|_| "Unable to represent `name` as a CString")?;
        let value_c = CString::new(value).map_err(|_| "Unable to represent `value` as a CString")?;
        let unit_c = CString::new(unit).map_err(|_| "Unable to represent `unit` as a CString")?;
        let mut handle = SensorValueHandle::default();
        match unsafe { uhd_sensor_value_make_from_string(&mut handle, name_c.as_ptr(), value_c.as_ptr(), unit_c.as_ptr()) } {
            0 => Ok(Self{ handle }),
            _ => Err("Nonzero return value from C API call in SensorValue::from_string")
        }
    }

    pub fn from_int(name:&str, value:i32, unit:&str, formatter:&str) -> Result<Self, &'static str> {
        let name_c = CString::new(name).map_err(|_| "Unable to represent `name` as a CString")?;
        let unit_c = CString::new(unit).map_err(|_| "Unable to represent `unit` as a CString")?;
        let formatter_c = CString::new(formatter).map_err(|_| "Unable to represent `formatter` as a CString")?;
        let mut handle = SensorValueHandle::default();
        match unsafe { uhd_sensor_value_make_from_int(&mut handle, name_c.as_ptr(), value, unit_c.as_ptr(), formatter_c.as_ptr()) } {
            0 => Ok(Self{ handle }),
            _ => Err("Nonzero return value from C API call in SensorValue::from_int")
        }
    }

    pub fn from_realnum(name:&str, value:f64, unit:&str, formatter:&str) -> Result<Self, &'static str> {
        let name_c = CString::new(name).map_err(|_| "Unable to represent `name` as a CString")?;
        let unit_c = CString::new(unit).map_err(|_| "Unable to represent `unit` as a CString")?;
        let formatter_c = CString::new(formatter).map_err(|_| "Unable to represent `formatter` as a CString")?;
        let mut handle = SensorValueHandle::default();
        match unsafe { uhd_sensor_value_make_from_realnum(&mut handle, name_c.as_ptr(), value, unit_c.as_ptr(), formatter_c.as_ptr()) } {
            0 => Ok(Self{ handle }),
            _ => Err("Nonzero return value from C API call in SensorValue::from_realnum")
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

    pub fn get_value(&self) -> Result<String, &'static str> {
        let mut buff:Vec<u8> = vec![0u8; 64];
        match unsafe { uhd_sensor_value_value(self.handle, buff.as_mut_ptr(), 64) } {
            0 => {
                let nonzero:Vec<u8> = buff.iter().take_while(|x| **x != 0).map(|x| *x).collect();
                Ok(String::from_utf8(nonzero).map_err(|_| "Unable to convert string returned from UHD as UTF-8")?)
            },
            _ => Err("Nonzero return value from C API call in SensorValue::get_value")
        }
    }

    pub fn get_unit(&self) -> Result<String, &'static str> {
        let mut buff:Vec<u8> = vec![0u8; 64];
        match unsafe { uhd_sensor_value_unit(self.handle, buff.as_mut_ptr(), 64) } {
            0 => {
                let nonzero:Vec<u8> = buff.iter().take_while(|x| **x != 0).map(|x| *x).collect();
                Ok(String::from_utf8(nonzero).map_err(|_| "Unable to convert string returned from UHD as UTF-8")?)
            },
            _ => Err("Nonzero return value from C API call in SensorValue::get_unit")
        }
    }

    pub fn to_pp_string(&self) -> Result<String, &'static str> {
        let mut buff:Vec<u8> = vec![0u8; 64];
        match unsafe { uhd_sensor_value_to_pp_string(self.handle, buff.as_mut_ptr(), 64) } {
            0 => {
                let nonzero:Vec<u8> = buff.iter().take_while(|x| **x != 0).map(|x| *x).collect();
                Ok(String::from_utf8(nonzero).map_err(|_| "Unable to convert string returned from UHD as UTF-8")?)
            },
            _ => Err("Nonzero return value from C API call in SensorValue::to_pp_string")
        }
    }

    pub fn last_error(&self) -> Result<String, &'static str> {
        let mut buff:Vec<u8> = vec![0u8; 64];
        match unsafe { uhd_sensor_value_last_error(self.handle, buff.as_mut_ptr(), 64) } {
            0 => {
                let nonzero:Vec<u8> = buff.iter().take_while(|x| **x != 0).map(|x| *x).collect();
                Ok(String::from_utf8(nonzero).map_err(|_| "Unable to convert string returned from UHD as UTF-8")?)
            },
            _ => Err("Nonzero return value from C API call in SensorValue::last_error")
        }
    }

    pub fn get_data_type(&self) -> Result<DataType, &'static str> {
        let mut ans = DataType::Boolean;
        match unsafe { uhd_sensor_value_data_type(self.handle, &mut ans) } {
            0 => Ok(ans),
            _ => Err("Nonzero return value from C API call in SensorValue::get_data_type"),
        }
    }

    pub fn to_bool(&self) -> Result<bool, &'static str> {
        let mut ans = false;
        match unsafe { uhd_sensor_value_to_bool(self.handle, &mut ans) } {
            0 => Ok(ans),
            _ => Err("Nonzero return value in SensorValue::to_bool")
        }
    }

    pub fn to_int(&self) -> Result<i32, &'static str> {
        let mut ans = 0;
        match unsafe { uhd_sensor_value_to_int(self.handle, &mut ans) } {
            0 => Ok(ans),
            _ => Err("Nonzero return value in SensorValue::to_int")
        }
    }

    pub fn to_realnum(&self) -> Result<f64, &'static str> {
        let mut ans = 0.0;
        match unsafe { uhd_sensor_value_to_realnum(self.handle, &mut ans) } {
            0 => Ok(ans),
            _ => Err("Nonzero return value in SensorValue::to_realnum")
        }
    }

}

impl std::ops::Drop for SensorValue {
    fn drop(&mut self) {
        unsafe{ uhd_sensor_value_free(&mut self.handle); }
    }
}