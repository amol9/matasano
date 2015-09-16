use std::char;

use common::{err, hex};


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


macro_rules! chr {
    ( $x : expr ) => ( ascii::u8_to_char( $x ) );
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


pub fn valid_chars() -> Vec<u8> {
    let mut chars = vec![ 9, 10, 13 ];
    for i in 32 .. 127 {
        chars.push(i); 
    }
    chars
}


// convert stdin input strings containing hex chars, like, \x01
pub fn scan_hex(input: &str) -> Result<String, err::Error> {
    let mut output = String::new();
    let mut char_it = input.chars();

    let mut c = char_it.next();
    while c != None {
        if c.unwrap() == '\\' {
            let (x, h1, h2) = (char_it.next(), char_it.next(), char_it.next());
            if ( x != None && x.unwrap() == 'x' && h1 != None && h2 != None ) {
                let xc = try!(hex::hex_char_to_u8(h1.unwrap())) << 4 | try!(hex::hex_char_to_u8(h2.unwrap()));
                output.push(u8_to_char(xc));
            } else {
                for _ in 0 .. 3 {
                    char_it.next_back();
                }
            }
        } else {
            output.push(c.unwrap());
        }
        c = char_it.next();
    }
    Ok(output)
}

