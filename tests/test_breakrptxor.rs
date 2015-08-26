extern crate matasano;
use self::matasano::common::util;
use self::matasano::set1::breakrptxor as brx;


macro_rules! match_res {
    ( $x : expr ) => ( match $x {
        Ok(v)   => v,
        Err(e)  => { println!("{}", e); assert!(false); return; }
    } );
}


#[test]
fn test_guess_key_length() {
    let data_filepath = "data/6b.txt";
    let text = match_res!(util::read_file_to_str(&data_filepath));
    let keylength = match_res!(brx::guess_key_length(&text));

    println!("keylength = {}", keylength);
}


#[test]
fn test_guess_key() {
    let data_filepath = "data/6.txt";
    let text = match_res!(util::read_file_to_str(&data_filepath));
    let keylength = match_res!(brx::guess_key_length(&text));

    let key = match_res!(brx::guess_key(&text, keylength));

    println!("key = {}", key);
}


#[test]
fn test_break_cipher() {
    let data_filepath = "data/6b.txt";
    let plaintext = match_res!(brx::break_cipher(&data_filepath));

    println!("{}", plaintext);
}


