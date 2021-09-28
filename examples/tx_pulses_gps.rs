
use std::collections::HashSet;

use uhd_rs::usrp::USRP;

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


    Ok(())
}