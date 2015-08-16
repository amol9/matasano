use std::char;
use std::io;

use common::{err, hex};

//pub fn hex_to_raw(input: &str) -> Result<Vec<u8>, FromHexError> {
//	return input.from_hex();
//}


// base64 encoding:
// 00 - 25 : A - Z
// 26 - 51 : a - z
// 52 - 61 : 0 - 9
// 62 : +
// 63 : /
fn base64_lookup(index: u8) -> Result<char, err::Error> {
	let ord: u8 = match index {
		0...25	=> index + 65,
		26...51	=> index + 71,
		52...61	=> index - 4,
		62	=> 43,
		63	=> 47,
		_	=> return Err(err::make_error(format!("base64 index out of range: {}", index))),
	};

	match char::from_u32(ord as u32) {
		Some(v)	=> Ok(v),
		None	=> Err(err::make_error(format!("bad ordinal: {}", ord))),
	}
}


fn debug_print(msg: &str) {
	println!("{}", msg);
}


pub fn raw_to_base64(input: Vec<u8>) -> Result<String, err::Error> {
	let mut index: usize	= 0;
	let mut v		= Vec::new();
	let mut output		= String::new();
	let mut pad: usize	= 0;

	while index < input.len() {
		if (index + 3 > input.len()) {
			pad =  (index + 3) - input.len();

			for i in 0..(3-pad) {
				v.push(input[index+i]);
			}

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
			output.push_str("==");
			break;
		}

		b64index = v[1] & 0xF;	//??
		b64index = b64index << 2 | v[2] >> 6;

		b64char = try!(base64_lookup(b64index));
		output.push(b64char);

		if pad == 1 {
			output.push_str("=");
			break;
		}

		b64index = v[2] & 0x3F;
		b64char = try!(base64_lookup(b64index));
		output.push(b64char);

		v.clear();
	}
				
	return Ok(output);
}


pub fn hex_to_base64(input: &str) -> Result<String, err::Error> {
	let r: Result<Vec<u8>, err::Error> = hex::hex_to_raw(input);

	let raw: Vec<u8> = match r {
		Ok(v)	=> v,
		Err(e)	=> return Err(err::make_error(String::from("error"))),
	};

	let r: Result<String, err::Error> = raw_to_base64(raw);

	match r {
		Ok(v)	=> Ok(v),
		Err(e)	=> Err(err::make_error(String::from("error"))),
	}
}


pub fn interactive() -> u32 {
	let mut input = String::new();

	println!("enter a hex number: ");
	io::stdin().read_line(&mut input);

	match hex_to_base64(&input) {
		Ok(v)	=> println!("{}", v),
		Err(e)	=> println!("{}", e),
	};
	0
}

