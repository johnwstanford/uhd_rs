
use crate::usrp::USRP;

pub mod simple_rx;

pub trait Job<T> {

	// A type that implements Job<T> takes a mutable reference to a USRP, uses it
	// to do work, then returns a Result with an owned T, which represents some kind 
	// of product resulting from the job
	fn execute(&self, usrp:&mut USRP) -> Result<T, &'static str>;
	
	// We also want a Job<T> to be able to produce a String describing itself because
	// there will be a lot of times when we want to save the result to a file or do something
	// like that and we want to be able to describe the job somehow.
	// TODO: consider just requiring Self to implement std::fmt::Display instead
	fn descriptor(&self) -> String;

	// For now, I'm going to require Self to provide the sample rate, but this is likely to
	// go away at some point
	fn sample_rate_sps(&self) -> f64;

}