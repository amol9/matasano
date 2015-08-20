extern crate matasano;
use self::matasano::set1::xorcipher;

#[test]
fn test_cryptopals_case() {
    let cipher = String::from("0101");
    let guess: String = match xorcipher::decipher(&cipher) {
        Ok(v)   => v,
        Err(e)  => String::from("error"),
    };

    println!("guess: {}", guess);
}
