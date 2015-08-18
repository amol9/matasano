use std::error;
use std::fmt;
use std::convert;


#[derive(Debug)]
pub struct Error {
	cause: String,
}


impl error::Error for Error {
	fn description(&self) -> &str {
		&self.cause
	}
}


impl fmt::Display for Error {
	fn fmt(&self, fmtr: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		println!("{}", self.cause);
		Ok(())
	}
}

impl convert::From<Error> for String {
	fn from(e: Error) -> Self {
		e.cause
	}
}


pub fn make_error(msg: String) -> Error {
	Error {cause: msg}
}


pub enum ExitCode {
	Success = 0,
	Error
}
