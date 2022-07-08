
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

    println!("Waiting for reference lock and GPS lock...");
    for _ in 0..30 {
        let ref_locked:bool = usrp.get_mboard_sensor("ref_locked", 0)?.to_bool()?;
        let gps_locked:bool = usrp.get_mboard_sensor("gps_locked", 0)?.to_bool()?;
        if ref_locked && gps_locked {
            break;
        } else {
            println!("GPS: {}, Ref: {}", ref_locked, gps_locked);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    // Set to GPS time
    let gps_time = usrp.get_mboard_sensor("gps_time", 0)?.to_int()?;
    usrp.set_time_next_pps(gps_time as i64 + 1, 0.0, 0)?;

    // Wait for it to apply
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Check times
    let gps_time = usrp.get_mboard_sensor("gps_time", 0)?.to_int()?;
    let time_last_pps = usrp.get_time_last_pps(0)?;

    println!("GPS Time: {:?}", gps_time);
    println!("USRP Time: {:?}", time_last_pps);

    Ok(())
}