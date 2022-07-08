use std::collections::HashSet;
use crate::usrp::USRP;

pub fn sync_to_gps(usrp: &mut USRP, print_status: bool) -> Result<(), &'static str> {

    let time_sources:HashSet<String> = usrp.get_time_sources(0)?.into_iter().collect();
    let clock_sources:HashSet<String> = usrp.get_clock_sources(0)?.into_iter().collect();
    let sensor_names:HashSet<String> = usrp.get_mboard_sensor_names(0)?.into_iter().collect();

    if !time_sources.contains("gpsdo") || !clock_sources.contains("gpsdo") {
        return Err("Synching to GPS requires a GPSDO");
    }

    if !sensor_names.contains("ref_locked") | !sensor_names.contains("gps_locked") {
        return Err("Sensors expected to include ref_locked and gps_locked");
    }

    usrp.set_time_source("gpsdo", 0)?;
    usrp.set_clock_source("gpsdo", 0)?;

    if print_status {
        println!("Waiting for reference lock and GPS lock...");
    }

    for _ in 0..30 {
        let ref_locked:bool = usrp.get_mboard_sensor("ref_locked", 0)?.to_bool()?;
        let gps_locked:bool = usrp.get_mboard_sensor("gps_locked", 0)?.to_bool()?;
        if ref_locked && gps_locked {
            break;
        } else {
            if print_status {
                println!("GPS: {}, Ref: {}", ref_locked, gps_locked);
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    // Set to GPS time
    // Note: this isn't GPS time-of-week; it's UTC time
    // provided by GPS
    let gps_time = usrp.get_mboard_sensor("gps_time", 0)?.to_int()?;
    usrp.set_time_next_pps(gps_time as i64 + 1, 0.0, 0)?;

    // Wait for it to apply
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Check times
    let gps_time = usrp.get_mboard_sensor("gps_time", 0)?.to_int()?;
    let time_last_pps = usrp.get_time_last_pps(0)?;

    if print_status {
        println!("GPS Time: {:?}", gps_time);
        println!("USRP Time: {:?}", time_last_pps);
    }

    if gps_time != time_last_pps.0 as i32 {
        Err("USRP and UTC time expected to be synched but aren't")
    } else {
        Ok(())
    }
}