
pub type UhdError = isize;

fn check_err<T>(t:T, result:isize) -> Result<T, &'static str> {
	match result {
		0 => Ok(t),
		_ => Err("Failed return value check")
	}
}

// Types and functions on top of the UHD library
pub mod job;


// Types and functions that directly reflect the UHD library
pub mod rx_streamer;
pub mod tx_streamer;
pub mod usrp;

pub mod types;