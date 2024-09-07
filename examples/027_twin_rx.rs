
use clap::{Arg, App};

use uhd_rs::usrp::{StreamCmd, StreamMode, USRP};

use uhd_rs::types::{TuneRequest, TuneRequestPolicy};
use std::ffi::CString;
use std::time::Duration;

const ALL_CHANS: [usize; 4] = [0, 1, 2, 3];
const DWELLS_PER_SEC: usize = 2;

const EXPORT_CHAN: usize = 0;

fn main() -> Result<(), &'static str> {

    let dwell_spacing_msg = format!("Number of dwells spaced {} [ms] apart", 1_000 / DWELLS_PER_SEC);

    let matches = App::new("Rx Example for UHD_rs")
        .version("0.1.0")
        .author("John Stanford (johnwstanford@gmail.com)")
        .about("Records raw IQ samples to a file")
        .arg(Arg::with_name("filename")
            .short("f").long("filename")
            .help("Output filename")
            .required(false).takes_value(true))
        .arg(Arg::with_name("sample_rate_sps")
            .short("s").long("sample_rate_sps")
            .takes_value(true).required(false))
        .arg(Arg::with_name("freq_hz")
            .long("freq_hz")
            .takes_value(true).required(false))
        .arg(Arg::with_name("gain_db")
            .long("gain_db")
            .takes_value(true))
        .arg(Arg::with_name("args")
            .long("args")
            .takes_value(true))
        .arg(Arg::with_name("time_sec")
            .long("time_sec")
            .help("Time to capture [seconds]")
            .takes_value(true).required(false))
        .arg(Arg::with_name("num_dwells")
            .long("num_dwells")
            .help(&dwell_spacing_msg)
            .takes_value(true).required(false))
        .get_matches();

    let rx_freq = matches.value_of("freq_hz").unwrap_or("531e6").parse().unwrap();
    let rx_rate = matches.value_of("sample_rate_sps").unwrap_or("2e6").parse().unwrap();
    let rx_gain = matches.value_of("gain_db").unwrap_or("93.0").parse().unwrap();
    let rx_time = matches.value_of("time_sec").unwrap_or("0.002").parse::<f64>().unwrap();
    let num_dwells = matches.value_of("num_dwells").unwrap_or("40").parse::<usize>().unwrap();

    let num_rx_samps = (rx_time * rx_rate) as usize;

    let mut usrp = USRP::new(matches.value_of("args").unwrap_or(""))?;
    let mut rx_subdev = usrp.get_subdev_spec(0)?;
    let n_rx_subdevs = rx_subdev.len()?;

    println!("Num subdevs: {} ({})", n_rx_subdevs, rx_subdev.to_string()?);
    for i in 0..n_rx_subdevs {
        println!("Subdev {}: {:?}", i, usrp.get_rx_subdev_name(i)?);
        println!("    LOs: {:?}", usrp.get_rx_lo_names(i)?.get_rust_vec()?);
        println!("    LO SRC Options: {:?}", usrp.get_rx_lo_sources("all", i)?.get_rust_vec()?);
        println!("    LO SRC: {:?}", usrp.get_rx_lo_source("all", i)?);
        println!("    LO Export Enabled: {:?}", usrp.get_rx_lo_export_enabled("all", i)?);
    }

    std::thread::sleep(Duration::from_millis(100));

    // Set up RX
    let empty_args = CString::new("").unwrap();

    let tune_request = TuneRequest {
        target_freq:    rx_freq,					// Target frequency for RF chain in Hz
        rf_freq_policy: TuneRequestPolicy::Auto, 	// RF frequency policy
        rf_freq: 		rx_freq,	    			// RF frequency in Hz
        dsp_freq_policy:TuneRequestPolicy::Auto, 	// DSP frequency policy
        dsp_freq:		0.0,						// DSP frequency in Hz
        args:empty_args.as_ptr()					// Key-value pairs delimited by commas
    };

    for channel in ALL_CHANS.iter() {
        usrp.set_rx_rate(rx_rate, *channel)?;
        usrp.set_rx_gain(rx_gain, *channel, "")?;
        let _rx_tune_result = usrp.set_rx_freq(&tune_request, *channel)?;

        let rx_rate_rb = usrp.get_rx_rate(*channel)?;
        let rx_gain_rb = usrp.get_rx_gain(*channel, "")?;
        let rx_freq_rb = usrp.get_rx_freq(*channel)?;

        println!(
            "CH{}: {:.2e} [sps], {:.1} [dB], {:.3} [MHz]",
            channel, rx_rate_rb, rx_gain_rb, rx_freq_rb / 1.0e6,
        );
    }
    
    std::thread::sleep(Duration::from_millis(50));

    usrp.set_rx_lo_export_enabled(true, "all", EXPORT_CHAN)?;
    usrp.set_rx_lo_source("internal", "all", EXPORT_CHAN)?;
    usrp.set_rx_lo_source("companion", "all", EXPORT_CHAN ^ 1)?;
    usrp.set_rx_lo_source("external", "all", EXPORT_CHAN ^ 2)?;
    usrp.set_rx_lo_source("external", "all", EXPORT_CHAN ^ 3)?;

    std::thread::sleep(Duration::from_millis(50));

    let (now_full, now_frac) = usrp.get_time_now(0)?;
    println!("Time now: {} = {}", now_full, now_frac);
    
    usrp.set_command_time(now_full+1, now_frac, 0)?;
    for channel in ALL_CHANS.iter() {
        let _rx_tune_result = usrp.set_rx_freq(&tune_request, *channel)?;
        println!("CH{}: Timed command complete", channel);
    }
    usrp.clear_command_time(0)?;

    std::thread::sleep(Duration::from_millis(50));

    for i in 0..n_rx_subdevs {
        println!("Subdev {}: {:?}", i, usrp.get_rx_subdev_name(i)?);
        println!("    LO SRC: {:?}", usrp.get_rx_lo_source("all", i)?);
        println!("    LO Export Enabled: {:?}", usrp.get_rx_lo_export_enabled("all", i)?);
    }

    // std::thread::sleep(Duration::from_secs(4));

    let mut rx_streamer = usrp.get_rx_stream("", &ALL_CHANS)?;

    let (now_full, _) = usrp.get_time_now(0)?;

    for i in 0..num_dwells {

        let stream_cmd_start = StreamCmd{
            stream_mode: StreamMode::NumSampsAndDone,
            num_samps: num_rx_samps,
            stream_now: false,
            time_spec_full_secs: now_full + 2 + (i / DWELLS_PER_SEC) as i64,
            time_spec_frac_secs: (i % DWELLS_PER_SEC) as f64 * (1.0 / DWELLS_PER_SEC as f64),
        };
        rx_streamer.stream(&stream_cmd_start)?;

        let mut rx_buffer0: Vec<(i16, i16)> = vec![(0,0); num_rx_samps];
        let mut rx_buffer1: Vec<(i16, i16)> = vec![(0,0); num_rx_samps];
        let mut rx_buffer2: Vec<(i16, i16)> = vec![(0,0); num_rx_samps];
        let mut rx_buffer3: Vec<(i16, i16)> = vec![(0,0); num_rx_samps];

        let (num_samps, rx_time_spec) = rx_streamer.recv_one_multi_chan(
            &mut [&mut rx_buffer0, &mut rx_buffer1, &mut rx_buffer2, &mut rx_buffer3]
        )?;

        println!("{} samples received at {:?}", num_samps, rx_time_spec);

        if num_samps > 0 {
            for (ch, buff) in vec![("A0", &rx_buffer0), ("A1", &rx_buffer1), ("B0", &rx_buffer2), ("B1", &rx_buffer3)] {
                let filename = matches.value_of("filename")
                    .map(|s| s.to_owned())
                    .unwrap_or(format!("twinrx{:04}_{}_{:.2}MHz_{}dB_{}Msps.bin", i, ch, rx_freq/1.0e6, rx_gain as usize, (rx_rate/1.0e6) as usize));

                uhd_rs::io::write_sc16_to_file(filename, buff)?;
            }
        }
    }

    Ok(())
}
