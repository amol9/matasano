use std::io;
use std::ascii::AsciiExt;

use err;

pub fn hex_to_raw<'a>(input: &str) -> Result<Vec<u8>, err::Error<'a>> {
	if (input.len() % 2 == 1) {
		return Err(err::make_error(&String::from("need an even number of hex digits")));
	}

	let index: usize = 0;
	let raw: Vec<u8> = Vec::new();

	while (index < input.len()) {
		let a: u8 = try!(hex_char_to_int(input.char_at(index)));
		let b: u8 = try!(hex_char_to_int(input.char_at(index + 1)));

		let i: u8 = a << 4 | b;
		raw.push(i);
	}

	Ok(raw)
}


pub fn hex_char_to_int<'a>(hex_char: char) -> Result<u8, err::Error<'a>> {
	if (hex_char >= '0' && hex_char <= '9') {
		return Ok(hex_char as u8 - 48);
	}

	hex_char.make_ascii_uppercase();

	if (hex_char >= 'A' && hex_char <= 'F') {
		return Ok(hex_char as u8 - 55);
	}

	Err(err::make_error(&format!("invalid hex digit {}", hex_char)))
}


pub fn raw_to_hex<'a>(raw: Vec<u8>) -> Result<String, Error<'a>> {
	
}
