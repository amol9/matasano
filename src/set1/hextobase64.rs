
use common::{base64, challenge, util};


pub static info: challenge::Info = challenge::Info {
    no:         1,
    title:      "Convert hex to base64",
    help:       "",
    execute_fn: interactive
};

pub fn interactive() -> i32 {
	let hex = rtry!(util::input("enter a hex number"), exit_err!());

	match base64::hex_to_base64(&hex.trim()) {
		Ok(v)	=> { println!("base64: {}", v); exit_ok!() }
		Err(e)	=> { println!("{}", e); exit_err!() }
	}
}

