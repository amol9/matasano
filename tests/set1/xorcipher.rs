use std::char;

extern crate matasano;
use self::matasano::common::{hex, ascii};
use self::matasano::common::cipher::one_byte_xor as obx;


#[test]
fn test_cryptopals_case() {
    let cipher = String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    match obx::guess_key(&r!(hex::hex_to_raw(&cipher)), None) {
        Ok(v)   => { assert_eq!(rts!(&v.plain), "Cooking MC's like a pound of bacon"); },
        Err(e)  => { println!("{}", e); assert!(false); }
    };
}

#[allow(dead_code)]
fn print_freq_list(fl: &Vec<f32>) {
    for i in 0..127 as usize {
        print!("{}: {:.*} ", char::from_u32(i as u32).unwrap(), 4, fl[i]);
        if i as u32 % 10 == 0 {
            println!("");
        }
    }
}

