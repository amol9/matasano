use std::char;

use common::err;


pub fn raw_to_str(raw: &Vec<u8>) -> Result<String, err::Error> {
    let mut result = String::new();

    for byte in raw {
        result.push(char::from_u32(*byte as u32).unwrap());
    }

    Ok(result)
    //mkerr!("sdkfjsd")
}


pub fn str_to_raw(input: &str) -> Result<Vec<u8>, err::Error> {
    let mut output: Vec<u8> = Vec::new();
    
    for c in input.chars() {
        output.push(c as u8);
    }
    Ok(output)
}

pub fn u8_to_char(input: u8) -> char {
    match char::from_u32(input as u32) {
        Some(v) => v,
        None    => '?'
    }
}


pub fn filter_whitespace(input: &str) -> Result<String, err::Error> {
    let mut output = String::new();
    for c in input.chars() {
        if !c.is_whitespace() {
            output.push(c);
        }
    }
    Ok(output)
}


macro_rules! rts {
    ( $x : expr ) => ( try!( ascii::raw_to_str( $x ) ) );
}


macro_rules! raw {
    ( $x : expr ) => ( try!( ascii::str_to_raw( $x ) ) );
}


macro_rules! strjoin {
    ( $ ( $x : expr ), * ) => (
        {
            let mut s = String::new();
            ( $ ( s.push_str($x) ), * );
            s
        } );
}


macro_rules! rawjoin {
    ( $ ( $x : expr ), * ) => (
        {
            let mut r = Vec::<u8>::new();
            ( $ ( r.extend($x) ), * );
            r
        } );
}


macro_rules! strn {
    ( $c : expr, $n : expr ) => ( 
        { 
            let mut s = String::new();
            for _ in 0 .. $n {
                s.push($c);
            }
            s
        } );
}

pub const valid_chars = [ 9, 10, 13, 32 .. 126 ];
