
use matasano::set2::breakaesecb2 as bae2;
use matasano::common::{base64, ascii};
use matasano::common::cipher::cipherbox as cb;


fn test(plain: &str) {
    let raw = r!(ascii::str_to_raw(&plain));
    let b64 = r!(base64::raw_to_base64(&raw));
    let mut cbox = r!(cb::init(&b64));
    cbox.enable_random_prefix(bae2::max_random_data_length);
    let p = r!(bae2::break_aes_ecb(&cbox));
    assert_eq!(p, plain);
}


#[ignore]   //time consuming
#[test]
fn test_break() {
    test("hello");
    test("1234567890123456");
    test("this is a simple one-line message");
    test("hi,\nthis is a multi-line message\nok\n");
    test("a");
}


#[test]
fn test_break_quick() {
    test("123456789012345678");
}

