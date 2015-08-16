use std::io;

use common::{err, hex};


pub fn xor_raw(x: Vec<u8>, y: Vec<u8>) -> Result<Vec<u8>, err::Error> {
	let mut result: Vec<u8> = Vec::new();

	let size: usize = x.len();

	for i in 0..size {
		result.push(x[i] ^ y[i]);
	}
	
	Ok(result)
}

//add size eq check
pub fn xor_hex(x: &str, y: &str) -> Result<String, err::Error> {
	let rx: Vec<u8> =  try!(hex::hex_to_raw(x));
	let ry: Vec<u8> =  try!(hex::hex_to_raw(y));

	let rr: Vec<u8> = try!(xor_raw(rx, ry));

	let result = try!(hex::raw_to_hex(rr));
	Ok(result)
}


pub fn interactive() -> u32 {
	println!("Fixed XOR (for hex numbers of equal length)");

	let mut x: String = String::new();

	println!("enter first hex number: ");
	io::stdin().read_line(&mut x);

	let mut y: String = String::new();

	println!("enter second hex number: ");
	io::stdin().read_line(&mut y);

	let result = String::new();

	match xor_hex(&x, &y) {
		Ok(v)	=> {
				println!("{}", v);
				0
			   },
		Err(e)	=> {
				println!("{}", e);
				1
			   }
	}
}
