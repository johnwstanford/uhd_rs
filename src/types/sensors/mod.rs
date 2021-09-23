
// From uhd/types/sensors.h

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

    fn uhd_sensor_value_make(h:*mut SensorValueHandle) -> isize;
    // uhd_error uhd_sensor_value_make_from_bool(uhd_sensor_value_handle* h, const char* name, bool value, const char* utrue, const char* ufalse);
    // uhd_error uhd_sensor_value_make_from_int(uhd_sensor_value_handle* h, const char* name, int value, const char* unit, const char* formatter);
    // uhd_error uhd_sensor_value_make_from_realnum(uhd_sensor_value_handle* h, const char* name, double value, const char* unit, const char* formatter);
    // uhd_error uhd_sensor_value_make_from_string(uhd_sensor_value_handle* h, const char* name, const char* value, const char* unit);

    fn uhd_sensor_value_free(h:*mut SensorValueHandle) -> isize;

    // uhd_error uhd_sensor_value_to_bool(uhd_sensor_value_handle h, bool *value_out);
    // uhd_error uhd_sensor_value_to_int(uhd_sensor_value_handle h, int *value_out);
    // uhd_error uhd_sensor_value_to_realnum(uhd_sensor_value_handle h, double *value_out);
    // uhd_error uhd_sensor_value_name(uhd_sensor_value_handle h, char* name_out, size_t strbuffer_len);
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

}

impl std::ops::Drop for SensorValue {
    fn drop(&mut self) {
        unsafe{ uhd_sensor_value_free(&mut self.handle); }
    }
}