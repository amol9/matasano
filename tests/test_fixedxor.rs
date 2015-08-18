extern crate matasano;
use self::matasano::set1::fixedxor;


#[test]
fn test_cryptopals_case() {
	let x = "1c0111001f010100061a024b53535009181c";
	let y = "686974207468652062756c6c277320657965";

	let r = "746865206b696420646f6e277420706c6179";

	match fixedxor::xor_hex(&x, &y) {
		Ok(v)	=> assert_eq!(v, r.to_uppercase()),
		Err(e)	=> {println!("{}", e); assert!(false);},
	}
}
