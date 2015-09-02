use std::io;

use common::{err, hex, xor, challenge};


pub static info: challenge::Info = challenge::Info {
    no:         4,
    title:      "",
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


pub fn interactive() -> i32 {
	println!("Fixed XOR (for hex numbers of equal length)");

	let mut x: String = String::new();

	println!("enter first hex number: ");
	io::stdin().read_line(&mut x);

	let mut y: String = String::new();

	println!("enter second hex number: ");
	io::stdin().read_line(&mut y);

	let result = String::new();

	match xor_hex(&x.trim(), &y.trim()) {
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
