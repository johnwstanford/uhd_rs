
use crate::types::sensors::{SensorValue, DataType};

#[test]
fn create_bool_sensor_value() {
    let x = SensorValue::from_bool("test_bool", true, "test_true", "test_false").unwrap();
    let x_name = x.get_name().unwrap();
    assert_eq!("test_bool", &x_name);
    assert_eq!(DataType::Boolean, x.get_data_type().unwrap());
    assert_eq!("true", x.get_value().unwrap());
    assert_eq!(true, x.to_bool().unwrap());
}

#[test]
fn create_string_sensor_value() {
    let x = SensorValue::from_string("test_string", "One Ring to Rule Them All", "unit_test").unwrap();
    assert_eq!("test_string", x.get_name().unwrap());
    assert_eq!(DataType::String, x.get_data_type().unwrap());
    assert_eq!("One Ring to Rule Them All", x.get_value().unwrap());

}

#[test]
fn create_int_sensor_value() {
    let x = SensorValue::from_int("temperature", 75, "[deg]", "%d").unwrap();
    assert_eq!("temperature", x.get_name().unwrap());
    assert_eq!(DataType::Integer, x.get_data_type().unwrap());
    assert_eq!("75", x.get_value().unwrap());
    assert_eq!(75, x.to_int().unwrap());
    assert_eq!("[deg]", x.get_unit().unwrap());
}

#[test]
fn create_realnum_sensor_value() {
    let x = SensorValue::from_realnum("pi", 3.14159, "", "%.3f").unwrap();
    assert_eq!("pi", x.get_name().unwrap());
    assert_eq!(DataType::RealNum, x.get_data_type().unwrap());
    assert_eq!("3.142", x.get_value().unwrap());
    assert_eq!(3.142, x.to_realnum().unwrap());
    assert_eq!("", x.get_unit().unwrap());
}

