use std::char;
use std::io;
use std::io::Write;

use common::{base64, challenge};


pub static info: challenge::Info = challenge::Info {
    no:         1,
    title:      "Convert hex to base64",
    help:       "",
    execute_fn: interactive
};


pub fn interactive() -> i32 {
	let mut hex = String::new();
	input!("enter a hex number: ", &mut hex);

	match base64::hex_to_base64(&hex.trim()) {
		Ok(v)	=> { println!("{}", v); exit_ok!() }
		Err(e)	=> { println!("{}", e); exit_err!() }
	}
}

