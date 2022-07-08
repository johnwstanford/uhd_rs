use std::ffi::CString;
use std::time::{Duration, Instant};
use uhd_rs::timing;
use uhd_rs::types::{TuneRequest, TuneRequestPolicy};
use uhd_rs::usrp::USRP;

fn main() -> Result<(), &'static str> {
    
    let mut usrp = USRP::new("")?;

    timing::sync_to_gps(&mut usrp, true)?;

    let channel = 0;
    let tx_freq0:f64 = 2000.5e6;
    let tx_rate:f64 = 1.0e6;
    let tx_gain:f64 = 30.0;
    let pulse_len_sec: f64 = 0.2;
    let chip_width_sec: f64 = 1.0e-3;

    // The chips are a way of giving the waveform some bandwidth
    let n_total: usize = (tx_rate * pulse_len_sec) as usize;
    let n_chip: usize = (tx_rate * chip_width_sec) as usize;
    let mut waveform: Vec<(i16, i16)> = vec![];
    while waveform.len() < n_total {
        waveform.append(&mut vec![(2000, 0); n_chip]);
        waveform.append(&mut vec![(-2000, 0); n_chip]);
    }

    // Set up TX
    let empty_args = CString::new("").unwrap();

    let tune_request0 = TuneRequest {
        target_freq:    tx_freq0,					// Target frequency for RF chain in Hz
        rf_freq_policy: TuneRequestPolicy::Auto, 	// RF frequency policy
        rf_freq: 		0.0,						// RF frequency in Hz
        dsp_freq_policy:TuneRequestPolicy::Auto, 	// DSP frequency policy
        dsp_freq:		0.0,						// DSP frequency in Hz
        args:empty_args.as_ptr()					// Key-value pairs delimited by commas
    };

    usrp.set_tx_rate(tx_rate, channel)?;
    usrp.set_tx_gain(tx_gain, channel, "")?;
    let tx_tune_result0 = usrp.set_tx_freq(&tune_request0, channel)?;
    println!("{:#?}", tx_tune_result0);

    println!("TX: {:.2e} [sps], {:.1} [dB], {:.3} [MHz]",
             usrp.get_tx_rate(channel)?,
             usrp.get_tx_gain(channel, "")?,
             usrp.get_tx_freq(channel)? / 1.0e6
    );

    let (t0_full_sec, _) = usrp.get_time_now(0)?;
    let t0 = Instant::now();

    // Create stream
    let mut tx_streamer = usrp.get_tx_stream::<i16, i16>("")?;

    // Start on a 5-second rollover
    let t0_full_sec = t0_full_sec - (t0_full_sec % 5);

    loop {

        let dt = t0.elapsed();
        let dt_next_full_sec = dt.as_secs() + 5;

        tx_streamer.single_coherent_pulse(
            &waveform,
            Some((t0_full_sec + dt_next_full_sec as i64, 0.0))
        )?;

        let sleep_until_dt = Duration::from_secs(dt_next_full_sec) - Duration::from_millis(100);
        while t0.elapsed() < sleep_until_dt {
            std::thread::sleep(Duration::from_millis(100));
        }
    }

}