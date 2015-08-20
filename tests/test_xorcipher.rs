use std::char;

extern crate matasano;
use self::matasano::set1::xorcipher;
use self::matasano::common::{hex, ascii};


#[test]
fn test_cryptopals_case() {
    let cipher = String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    let guess: String = match xorcipher::decipher(&cipher) {
        Ok(v)   => v,
        Err(e)  => {println!("{}", e); return} ,
    };

    println!("guess: {}", guess);
}


fn print_freq_list(fl: &Vec<f32>) {
    for i in 0..127 as usize {
        print!("{}: {:.*} ", char::from_u32(i as u32).unwrap(), 4, fl[i]);
        if i as u32 % 10 == 0 {
            println!("");
        }
    }
}


#[test]
fn test_compute_base_frequency() {
    let base_freq = match xorcipher::compute_base_frequency() {
        Ok(v)   => v,
        Err(e)  => return
    };
    print_freq_list(&base_freq);
}

#[test]
fn test_decrypt() {
    let cipher = String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    for i in 0..10 {
        let plain = match xorcipher::decrypt(&cipher, i) {
            Ok(v)   => v,
            Err(e)  => return,
        };
        println!("plain: {}", plain);
    }
}

#[test]
fn test_ach() {
    let cipher = String::from("ETAOIN SHRDLU");
    let raw: Vec<u8> = ascii::str_to_raw(&cipher).unwrap();
    let chex: String = hex::raw_to_hex(&raw).unwrap();

    let plain = match xorcipher::decipher(&chex) {
        Ok(v)   => v,
        Err(e)  => return,
    };
    println!("ach: {}", plain);
}
