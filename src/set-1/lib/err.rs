use std::error;
use std::fmt;


#[derive(Debug)]
pub struct Error<'a> {
	cause: &'a str,
}


impl<'a> error::Error for Error<'a> {
	fn description(&self) -> &str {
		&self.cause
	}
}


impl<'a> fmt::Display for Error<'a> {
	fn fmt(&self, fmtr: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		println!("{}", self.cause);
		Ok(())
	}
}


pub fn make_error(msg: &str) -> Error {
	Error {cause: msg}
}


pub enum ExitCode {
	Success = 0,
	Error
}
