use std::io;
use std::ascii::AsciiExt;

use common::err;

pub fn hex_to_raw(input: &str) -> Result<Vec<u8>, err::Error> {
	if (input.len() % 2 == 1) {
		return Err(err::make_error(String::from("need an even number of hex digits")));
	}

	let mut index: usize = 0;
	let mut raw: Vec<u8> = Vec::new();

	let mut chars_iter = input.chars();

	for i in 0..(input.len()/2) {
		let a: u8 = try!(hex_char_to_int(chars_iter.next().unwrap()));
		let b: u8 = try!(hex_char_to_int(chars_iter.next().unwrap()));

		let i: u8 = a << 4 | b;
		raw.push(i);
		index += 2;
	}

	Ok(raw)
}


pub fn hex_char_to_int(hex_char: char) -> Result<u8, err::Error> {
	if (hex_char >= '0' && hex_char <= '9') {
		return Ok(hex_char as u8 - 48);
	}

	let hc = hex_char.to_ascii_uppercase();

	if (hc >= 'A' && hc <= 'F') {
		return Ok(hc as u8 - 55);
	}

	Err(err::make_error(format!("invalid hex digit {}", hex_char)))
}


pub fn raw_to_hex(raw: Vec<u8>) -> Result<String, err::Error> {
	Ok(String::from("123"))	
}
