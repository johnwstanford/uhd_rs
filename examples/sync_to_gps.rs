
use std::collections::HashSet;
use uhd_rs::timing;

use uhd_rs::usrp::USRP;
use uhd_rs::types::sensors::DataType;

fn main() -> Result<(), &'static str> {

    let mut usrp = USRP::new("")?;

    let sensor_names:HashSet<String> = usrp.get_mboard_sensor_names(0)?.into_iter().collect();
    println!("Sensor names: {:#?}", sensor_names);

    for sensor in sensor_names.iter() {
        let sensor_value = usrp.get_mboard_sensor(sensor, 0)?;
        let sensor_type = sensor_value.get_data_type()?;
        match sensor_type {
            DataType::Boolean => println!("{}: {}", sensor, sensor_value.to_bool()?),
            DataType::Integer => println!("{}: {}", sensor, sensor_value.to_int()?),
            DataType::RealNum => println!("{}: {}", sensor, sensor_value.to_realnum()?),
            DataType::String  => println!("{}: {}", sensor, sensor_value.get_value()?),
        }
    }

    timing::sync_to_gps(&mut usrp, true)?;

    println!("Time source: {:?}", usrp.get_time_source(0)?);
    println!("Clock source: {:?}", usrp.get_clock_source(0)?);

    Ok(())
}