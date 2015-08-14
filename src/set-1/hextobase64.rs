extern crate rustc_serialize;

use self::rustc_serialize::hex::{FromHex, FromHexError};
use std::char;
use std::error;
use std::fmt;
use std::vec;


pub enum XXError {
	RawToBase64Error,
	Base64IndexError,
	HexToRawError,
	OrdinalError
}

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

pub fn hex_to_raw(input: &str) -> Result<Vec<u8>, FromHexError> {
	return input.from_hex();
}


// base64 encoding:
// 00 - 25 : A - Z
// 26 - 51 : a - z
// 52 - 61 : 0 - 9
// 62 : +
// 63 : /
fn base64_lookup(index: u8) -> Result<char, Error> {
	let ord: u8 = match index {
		0...25	=> index + 65,
		26...51	=> index + 71,
		52...61	=> index - 4,
		62	=> 43,
		63	=> 47,
		_	=> return Err(Error {cause: String::from("base64 index out of range")}),
	};

	match char::from_u32(ord as u32) {
		Some(v)	=> Ok(v),
		None	=> Err(Error {cause: String::from("bad ordinal")}),
	}
}

pub fn raw_to_base64(input: Vec<u8>) -> Result<String, Error> {
	let mut index: usize	= 0;
	let mut v		= Vec::new();
	let mut output		= String::new();
	let mut pad: usize	= 0;

	while index < input.len() {
		if (index + 3 > input.len()) {
			pad = input.len() - (index + 3);
			for i in 0..pad {
				v.push(0);
			}

		} else {
			v.push(input[index]);
			v.push(input[index + 1]);
			v.push(input[index + 2]);
		}
		index += 3;

		let mut b64index: u8 = v[0] >> 2;
		let mut b64char: char = try!(base64_lookup(b64index));
		output.push(b64char);


		b64index = v[0] << 6;
		b64index = b64index >> 2 | v[1] >> 4;
		b64char = try!(base64_lookup(b64index));
		output.push(b64char);

		if pad == 2 {
			break;
		}

		b64index = v[1] & 0xF;	//??
		b64index = v[1] << 2 | v[2] >> 6;
		b64char = try!(base64_lookup(b64index));
		output.push(b64char);

		if pad == 1 {
			break;
		}

		b64index = v[2] & 0x3F;
		b64char = try!(base64_lookup(b64index));
		output.push(b64char);
	}
				
	// if index + 3 < len(vec)
	// do padding
	// rem
	// get 4 b64 items by shifting
	// 
	// do lookup and find corr b64 chars
	// append to output
	// handle padding

	return Ok(output);
}


pub fn hex_to_base64(input: &str) -> Result<String, Error> {
	let r: Result<Vec<u8>, FromHexError> = hex_to_raw(input);

	let raw = match r {
		Ok(v)	=> v,
		Err(e)	=> return Err(Error {cause: String::from("error in converting hex to raw")}),
	};

	let r: Result<String, Error> = raw_to_base64(raw);

	match r {
		Ok(v)	=> Ok(v),
		Err(e)	=> Err(Error {cause: String::from("error in converting raw to base64")}),
	}
}

fn test_throw() -> Result<String, Error> {
	Err(Error {cause: String::from("test error") })
}

pub fn interactive() {

}

#[test]
fn test_two() {
	let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

	//let r: Result<String, Error> = test_throw();
	let r: Result<String, Error> = hex_to_base64(input);

	let base64 = match r {
		Ok(v)	=> v,
		Err(e)	=> {
				println!("{}", e);
				String::from("123")
			   }
	};

	println!("{}", base64);
}


fn print_raw(raw: Vec<u8>) {
	for i in raw {
		print!("{} ", i);
	}
	println!("");
}

#[test]
fn test_hex_to_raw() {
	let input = "00ff";
	let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
	match hex_to_raw(input) {
		Ok(v)	=> print_raw(v),
		Err(e)	=> println!("error: {}", e)
	}
}


