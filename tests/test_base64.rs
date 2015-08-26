extern crate matasano;
use self::matasano::common::{base64, hex, ascii};


macro_rules! m {
    ( $x : expr ) => ( match $x {
        Ok(v)   => v,
        Err(e)  => { println!("{}", e); assert!(false); return; }
    } );
}

macro_rules! s {
	( $x : expr ) => ( String::from($x) );
}


#[test]
fn test_str_to_base64() {
    let i = s!("ab");
    println!("{}", m!(base64::raw_to_base64(m!(ascii::str_to_raw(&i)))));
}


#[test]
fn test_base64_to_str() {
    assert_eq!(m!(ascii::raw_to_str(&m!(base64::base64_to_raw(&s!("aGVsbG8gYmFzZTY0ICEhIQ=="))))), s!("hello base64 !!!"));
    assert_eq!(m!(ascii::raw_to_str(&m!(base64::base64_to_raw(&s!("aGVsbG8="))))), s!("hello"));
    assert_eq!(m!(ascii::raw_to_str(&m!(base64::base64_to_raw(&s!("aGkgdGhlcmUg"))))), s!("hi there "));
    assert_eq!(m!(ascii::raw_to_str(&m!(base64::base64_to_raw(&s!("YQ=="))))), s!("a"));
    assert_eq!(m!(ascii::raw_to_str(&m!(base64::base64_to_raw(&s!("YWI="))))), s!("ab"));
}
