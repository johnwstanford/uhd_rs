
pub type UhdError = isize;

fn check_err<T>(t:T, result:isize) -> Result<T, &'static str> {
	match result {
		0 => Ok(t),
		_ => Err("Failed return value check")
	}
}

pub mod c_interop;
pub mod io;

pub mod error;

pub mod rx_streamer;
pub mod tx_streamer;
pub mod usrp;

pub mod types;

pub mod timing;