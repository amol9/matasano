use std::char;
use std::io;

use common::{base64, challenge};


pub static info: challenge::Info = challenge::Info {
    no:         1,
    title:      "",
    help:       "",
    execute_fn: interactive
};


pub fn interactive() -> i32 {
	let mut input = String::new();

	println!("enter a hex number: ");
	io::stdin().read_line(&mut input);

	match base64::hex_to_base64(&input) {
		Ok(v)	=> println!("{}", v),
		Err(e)	=> println!("{}", e),
	};
	0
}

