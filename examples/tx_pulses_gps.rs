
use std::collections::HashSet;

use uhd_rs::usrp::USRP;
use uhd_rs::types::sensors::DataType;

fn main() -> Result<(), &'static str> {
    
    let mut usrp = USRP::new("")?;

    let time_sources:HashSet<String> = usrp.get_time_sources(0)?.into_iter().collect();
    let clock_sources:HashSet<String> = usrp.get_clock_sources(0)?.into_iter().collect();
    let sensor_names:HashSet<String> = usrp.get_mboard_sensor_names(0)?.into_iter().collect();
    assert!(time_sources.contains("gpsdo"));
    assert!(clock_sources.contains("gpsdo"));

    usrp.set_time_source("gpsdo", 0)?;
    usrp.set_clock_source("gpsdo", 0)?;

    println!("Time source: {:?}", usrp.get_time_source(0)?);
    println!("Clock source: {:?}", usrp.get_clock_source(0)?);
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

    Ok(())
}