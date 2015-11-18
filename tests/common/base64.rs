extern crate matasano;
use self::matasano::common::{base64, ascii};


#[test]
fn test_str_to_base64() {
    let i = "ab";
    println!("{}", r!(base64::raw_to_base64(&r!(ascii::str_to_raw(&i)))));
}


#[test]
fn test_base64_to_str() {
    assert_eq!(r!(ascii::raw_to_str(&r!(base64::base64_to_raw("aGVsbG8gYmFzZTY0ICEhIQ==")))), "hello base64 !!!");
    assert_eq!(r!(ascii::raw_to_str(&r!(base64::base64_to_raw("aGVsbG8=")))), "hello");
    assert_eq!(r!(ascii::raw_to_str(&r!(base64::base64_to_raw("aGkgdGhlcmUg")))), "hi there ");
    assert_eq!(r!(ascii::raw_to_str(&r!(base64::base64_to_raw("YQ==")))), "a");
    assert_eq!(r!(ascii::raw_to_str(&r!(base64::base64_to_raw("YWI=")))), "ab");
}

