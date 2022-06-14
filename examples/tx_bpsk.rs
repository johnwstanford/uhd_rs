
use std::ffi::CString;
use std::time::{Duration, Instant};

use uhd_rs::types::{TuneRequest, TuneRequestPolicy};
use uhd_rs::usrp::USRP;

const BURST_LEN: Duration = Duration::from_secs(4);
const LEAD_TIME: Duration = Duration::from_millis(500);
const LOOP_TIME: Duration = Duration::from_millis(100);

fn main() -> Result<(), &'static str> {

    // TX parameters
    let tx_freq0:f64 = 2000.5e6;
    let tx_rate:f64 = 1.0e6;
    let tx_gain:f64 = 30.0;

    let chip_width_sec: f64 = 200.0e-6;
    let if_freq_hz: f64 = 0.0;

    let channel = 0;

    let empty_args = CString::new("").unwrap();

    let mut usrp = USRP::new("")?;

    println!("TX num channels: {}", usrp.tx_num_channels()?);

    // Set up TX
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

    println!("TX: {:.2e} [sps], {:.1} [dB], {:.3} [MHz]", usrp.get_tx_rate(channel)?, usrp.get_tx_gain(channel, "")?, usrp.get_tx_freq(channel)? / 1.0e6);

    // Create stream
    let mut tx_streamer = usrp.get_tx_stream::<i16, i16>("")?;

    // Set up BURST_LEN of waveform before starting stream
    let num_tx_samps: usize = (BURST_LEN.as_secs_f64() * tx_rate) as usize;
    let omega_rad_per_sec: f64 = if_freq_hz * 2.0 * std::f64::consts::PI;
    let dt:f64 = 1.0 / tx_rate;

    let mut buffer:Vec<(i16, i16)> = vec![];

    while buffer.len() < num_tx_samps {

        let t: f64 = dt*(buffer.len() as f64);
        let phase:f64 = omega_rad_per_sec * t;

        buffer.push(match (t/chip_width_sec) as usize % 2 {
            0 => ((phase.cos() *  8.0e3) as i16, (phase.sin() *  8.0e3) as i16),
            _ => ((phase.cos() * -8.0e3) as i16, (phase.sin() * -8.0e3) as i16)
        });

    }

    let t0 = Instant::now();
    let mut next_burst = t0.elapsed() + BURST_LEN - LEAD_TIME;

    loop {

        tx_streamer.single_coherent_pulse(&buffer, None).unwrap();

        while t0.elapsed() < next_burst {
            std::thread::sleep(LOOP_TIME);
        }

        next_burst += BURST_LEN;

    }

}