use std::env;
use std::process;
use std::convert::AsRef;

#[path = "set-1/hextobase64.rs"]
mod hextobase64;


fn main() {
	
	let problem = match env::args().nth(1) {
		Some(v)	=> v,
		None	=> {
				println!("please specify a problem to try..");
				process::exit(1);
			   }
	};

	match problem.as_ref() {
		"hextobase64"	=> hextobase64::interactive(),
		_		=> unreachable!("error"),
	}
}
