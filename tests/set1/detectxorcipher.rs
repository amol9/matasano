extern crate matasano;
use self::matasano::set1::detectxorcipher as dxc;


#[test]
fn test_cryptopals_case() {
    let data_filepath =  String::from("data/cryptopals_4.txt");

    
    let input = match dxc::read_input_file(&data_filepath) {
        Ok(v)   => v,
        Err(e)  => { println!("{}", e); assert!(false); return; }
    };

    let output = match dxc::detect_xor_cipher(&input) {
        Ok(v)   => v,
        Err(e)  => { println!("{}", e); assert!(false); return; }
    };

    assert_eq!(output, String::from("7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f")); 
}

