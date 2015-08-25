extern crate matasano;
use self::matasano::common::{util, breakrptxor};


macro_rules! match_res {
    ( $x : expr ) => ( match $x {
        Ok(v)   => v,
        Err(e)  => { println!("{}", e); assert!(false); return; }
    } );
}


#[test]
fn test_guess_key_length() {
    let data_filepath = "data/6.txt";
    let text = match_res!(util::read_file_to_str(&data_filepath));
    let keylength = match_res!(breakrptxor::guess_key_length(&text));

    println!("keylength = {}", keylength);
}
