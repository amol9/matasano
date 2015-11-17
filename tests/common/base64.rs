extern crate matasano;
use self::matasano::common::{base64, hex, ascii};


#[test]
fn test_str_to_base64() {
    let i = "ab";
    println!("{}", m!(base64::raw_to_base64(&m!(ascii::str_to_raw(&i)))));
}


#[test]
fn test_base64_to_str() {
    assert_eq!(m!(ascii::raw_to_str(&m!(base64::base64_to_raw("aGVsbG8gYmFzZTY0ICEhIQ==")))), "hello base64 !!!");
    assert_eq!(m!(ascii::raw_to_str(&m!(base64::base64_to_raw("aGVsbG8=")))), "hello");
    assert_eq!(m!(ascii::raw_to_str(&m!(base64::base64_to_raw("aGkgdGhlcmUg")))), "hi there ");
    assert_eq!(m!(ascii::raw_to_str(&m!(base64::base64_to_raw("YQ==")))), "a");
    assert_eq!(m!(ascii::raw_to_str(&m!(base64::base64_to_raw("YWI=")))), "ab");
}

