use std::ascii::AsciiExt;
use std::char;

use common::err;


pub struct Lower;
pub struct Upper;


pub trait Convert {
    fn u8_to_hex(n: u8) -> Option<char>;
}

impl Convert for Lower {
    fn u8_to_hex(n: u8) -> Option<char> {
        char::from_u32(n as u32 + 87)
    }
}

impl Convert for Upper {
    fn u8_to_hex(n: u8) -> Option<char> {
        char::from_u32(n as u32 + 55)
    }
}

pub fn hex_to_raw(input: &str) -> Result<Vec<u8>, err::Error> {
	if (input.len() % 2) == 1 {
		return Err(err::make_error(String::from("need an even number of hex digits")));
	}

	let mut raw: Vec<u8> = Vec::new();
	let mut chars_iter = input.chars();

	for _ in 0..(input.len()/2) {
		let a: u8 = try!(hex_char_to_u8(chars_iter.next().unwrap()));
		let b: u8 = try!(hex_char_to_u8(chars_iter.next().unwrap()));

		let i: u8 = a << 4 | b;
		raw.push(i);
	}

	Ok(raw)
}

pub fn hex_char_to_u8(hex_char: char) -> Result<u8, err::Error> {
	if hex_char >= '0' && hex_char <= '9' {
		return Ok(hex_char as u8 - 48);
	}

	let hc = hex_char.to_ascii_uppercase();

	if hc >= 'A' && hc <= 'F' {
		return Ok(hc as u8 - 55);
	}

	Err(err::make_error(format!("invalid hex digit {}", hex_char)))
}

pub fn u8_to_hex_char<T>(dec: u8) -> Result<char, err::Error> where T: Convert {
	if dec > 15 {
		return Err(err::make_error(String::from("must be a 4-bit decimal")));
	}

	let hex_char: char = match dec {
		0 ... 9 	=> char::from_u32(dec as u32 + 48).unwrap(),
		10 ... 15	=> T::u8_to_hex(dec).unwrap(), 
		_		=> return Err(err::make_error(String::from(format!("unreachable, dec: {}", dec))))
	};
	Ok(hex_char)
}

pub fn raw_to_hex<T>(raw: &Vec<u8>) -> Result<String, err::Error> where T: Convert {
	let mut result: String = String::new();

	for n in raw {
		result.push(try!(u8_to_hex_char::<T>(n >> 4)));
		result.push(try!(u8_to_hex_char::<T>(n & 0xF)));
	}
	Ok(result)
}

macro_rules! htr {
    ( $x : expr ) => ( try!( hex::hex_to_raw( $x ) ) );
}

macro_rules! rth {
    ( $x : expr ) => ( try!( hex::raw_to_hex( $x ) ) );
}

