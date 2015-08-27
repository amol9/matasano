use std::env;
use std::process;
use std::convert::AsRef;

extern crate matasano;
use self::matasano::set1::{hextobase64, fixedxor, xorcipher, detectxorcipher, rptxorcipher, breakrptxor};


fn main() {
	
	let problem = match env::args().nth(1) {
		Some(v)	=> v,
		None	=> {
				println!("please specify a problem to try..");
				process::exit(1);
			   }
	};

	match problem.as_ref() {
		"hextobase64"	    => hextobase64::interactive(),
		"fixedxor"	        => fixedxor::interactive(),
        "xorcipher"         => xorcipher::interactive(),
        "detectxorcipher"   => detectxorcipher::interactive(),
        "rptxorcipher"      => rptxorcipher::interactive(),
        "breakrptxor"       => breakrptxor::interactive(),
		_		            => panic!("error")
	};
}
