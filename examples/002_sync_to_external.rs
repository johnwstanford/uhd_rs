use uhd_rs::timing;

use uhd_rs::usrp::USRP;

fn main() -> Result<(), &'static str> {

    let mut usrp = USRP::new("")?;

    timing::sync_to_external(&mut usrp, true)?;

    println!("Time source: {:?}", usrp.get_time_source(0)?);
    println!("Clock source: {:?}", usrp.get_clock_source(0)?);

    Ok(())
}
