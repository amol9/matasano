use std::io;
use std::io::Write;

use common::{err, hex, xor, challenge};


pub static info: challenge::Info = challenge::Info {
    no:         2,
    title:      "Fixed XOR",
    help:       "",
    execute_fn: interactive
};


pub fn xor_hex(x: &str, y: &str) -> Result<String, err::Error> {
	if (x.len() != y.len()) {
		return Err(err::make_error(String::from("unequal number of hex digits")));
	}

	let rx: Vec<u8> =  try!(hex::hex_to_raw(x));
	let ry: Vec<u8> =  try!(hex::hex_to_raw(y));

	let rr: Vec<u8> = try!(xor::xor(&rx, &ry));

	let result = try!(hex::raw_to_hex::<hex::lower>(&rr));
	Ok(result)
}


pub fn interactive() -> err::ExitCode {
	let mut x: String = String::new();
	input!("enter first hex number: ", &mut x);

	let mut y: String = String::new();
	input!("enter second hex number: ", &mut y);

	match xor_hex(&x.trim(), &y.trim()) {
		Ok(v)	=> { println!("xor result: {}", v); exit_ok!() },
		Err(e)	=> { println!("{}", e); exit_err!() }
	}
}

