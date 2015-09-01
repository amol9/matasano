use std::env;
use std::process;
use std::convert::AsRef;

extern crate matasano;
use self::matasano::set1::{hextobase64, fixedxor, xorcipher, detectxorcipher, rptxorcipher, breakrptxor, aesdecrypt, detectaesecb};
use self::matasano::set2::{pkcs7, aescbc, aesoracle};


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
        "aesdecrypt"        => aesdecrypt::interactive(),
        "detectaesecb"      => detectaesecb::interactive(),

        "pkcs7"             => pkcs7::interactive(),
        "aescbc"            => aescbc::interactive(),
        "aesoracle"         => aesoracle::interactive(),

		_		            => panic!("error")
	};
}
