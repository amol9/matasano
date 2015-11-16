use std::error;
use std::fmt;
use std::convert;


#[derive(Debug)]
pub enum Type {
    Default,
    Padding
}


#[derive(Debug)]
pub struct Error {
	pub cause:      String,
    pub errtype:    Type
}


impl error::Error for Error {
	fn description(&self) -> &str {
		&self.cause
	}
}


impl fmt::Display for Error {
    #[allow(unused_variables)]
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
	Error {
        cause:      msg,
        errtype:    Type::Default}
}


pub fn make(msg: String, errtype: Type) -> Error {
	Error {
        cause:      msg,
        errtype:    errtype}
}


macro_rules! mkerr {
    ( $x : expr )               => ( Err(err::make(String::from($x), err::Type::Default)) );
    ( $x : expr, $y : expr )    => ( Err(err::make(String::from($x), $y)) );
}


macro_rules! etry {
    ( $expr : expr , $msg : expr ) => ( match $expr {
                            Ok(v)   => { v },
                            Err(e)  => { println!("{}", e); return mkerr!($msg) },
                            } );
}


macro_rules! rtry {
    ( $expr : expr , $ret : expr ) => ( match $expr {
                            Ok(v)   => { v },
                            Err(e)  => { println!("{}", e); return $ret }
                            } );
}


macro_rules! ctry {
    ( $cond : expr , $msg : expr )  => (
        if $cond {
           return mkerr!($msg);
        } );
}


macro_rules! ertry {
    ( $x : expr ) => (
        match $x {
            Ok(v)  => v,
            Err(e) => return e
        } );
}


//pub enum ExitCode {
//	Success = 0,
//	Error
//}

pub type ExitCode = i32;


macro_rules! exit_ok {
    ( ) => ( 0 );
}


macro_rules! exit_err {
    ( ) => ( 1 );
}

