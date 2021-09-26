
use crate::types::sensors::SensorValue;

#[test]
fn create_bool_sensor_value() {
    let x = SensorValue::from_bool("test_bool", true, "test_true", "test_false").unwrap();
    let x_name = x.get_name().unwrap();
    assert_eq!("test_bool", &x_name);
}