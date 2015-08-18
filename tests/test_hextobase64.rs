extern crate matasano;
use self::matasano::common::{base64, err};


fn test_hex_to_base64(input: &str, output: &str) {
	let r: Result<String, err::Error> = base64::hex_to_base64(input);

	let base64 = match r {
		Ok(v)	=> v,
		Err(e)	=> {
				println!("{}", e);
				String::from("N.A.")
			   }
	};

	assert_eq!(base64, output);
}


#[test]
fn test_cryptopals_case() {
	let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
	let output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
	test_hex_to_base64(input, output);
}


#[test]
fn test_more() {
	test_hex_to_base64("00", "AA==");
	test_hex_to_base64("00FF", "AP8=");
	test_hex_to_base64("00FFed", "AP/t");
	test_hex_to_base64("00F3ed45", "APPtRQ==");
	test_hex_to_base64("00F3ed455727efd982a8b340", "APPtRVcn79mCqLNA");
	test_hex_to_base64("", "");
}




