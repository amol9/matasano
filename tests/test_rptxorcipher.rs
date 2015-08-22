extern crate matasano;
use self::matasano::common::cipher::rpt_key_xor as rkx;


#[test]
fn test_cryptopals_case() {
	let plain = String::from("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal");
    let key = String::from("ICE");
	let cipher = String::from("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272\
        a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");

    match rkx::encrypt_str(&plain, &key) {
        Ok(v)   => assert_eq!(v, cipher),
        Err(e)  => { println!("{}", e); assert!(false); }
    }
}
