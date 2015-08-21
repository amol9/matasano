use std::char;

extern crate matasano;
use self::matasano::set1::xorcipher;
use self::matasano::common::{hex, ascii};


#[test]
fn test_cryptopals_case() {
    let cipher = String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    match xorcipher::decipher(&cipher) {
        Ok(v)   => { assert_eq!(v, "Cooking MC's like a pound of bacon"); },
        Err(e)  => { println!("{}", e); assert!(false); }
    };
}


fn print_freq_list(fl: &Vec<f32>) {
    for i in 0..127 as usize {
        print!("{}: {:.*} ", char::from_u32(i as u32).unwrap(), 4, fl[i]);
        if i as u32 % 10 == 0 {
            println!("");
        }
    }
}


//#[test]
fn test_decrypt_bf() {
    let cipher = String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    for i in 0..10 {
        let plain = match xorcipher::decrypt(&cipher, i) {
            Ok(v)   => v,
            Err(e)  => return,
        };
        println!("plain: {}", plain);
    }
}

