pub fn xor_raw(x: Vec<u8>, y: Vec<u8>) -> Result<Vec<u8>, Error> {
	let result: Vec::new();

	for i, j in x, y {
		result.push(i.bitxor(j));
	}
	
	result	
}

pub fn xor_hex(x: &str, y: &str, result: &mut str) -> Result<(), Error> {
	let rx: Vec<u8> =  try!(hex_to_raw(x));
	let ry: Vec<u8> =  try!(hex_to_raw(y));

	let rr: Vec<u8> = try!(xor_raw(rx, ry));

	result.push_str(try!(raw_to_hex(rr)));
	Ok(())
}


pub interactive() -> u32 {
	println!("Fixed XOR (for hex numbers of equal length)");

	let x: String::new();

	println!("enter first hex number: ");
	io::stdin().read_line(&mut x);

	let y: String::new();

	println!("enter second hex number: ");
	io::stdin().read_line(&mut y);

	let result = String::new();

	match xor_hex(&x, &y, &result) {
		Ok()	=> {
				println!("{}", result);
				0
			   },
		Err(e)	=> {
				println!("{}", e);
				1
			   }
	}
}
