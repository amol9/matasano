
use matasano::common::cipher::padding;
use matasano::common::ascii;


#[test]
fn test_pkcs7_unpad() {
    fn test(input: &str, bs: usize, exp: &str) -> bool {
        let ir = rr!(ascii::str_to_raw(&input), false);
        let out = rr!((padding::Pkcs7.unpad_fn)(&ir, bs), false);
        let outs = rr!(ascii::raw_to_str(&out), false);
        assert_eq!(outs, exp);
        true
    }

    assert!(test("ABCDE\x03\x03\x03", 8, "ABCDE"));
    assert!(test("ABCDEFG\x01", 8, "ABCDEFG"));
    assert!(test("ABCDEFG\x01", 4, "ABCDEFG"));
}


#[test]
fn test_pkcs7_pad() {
    fn test(input: &str, bs: usize, exp: &str) -> bool {
        let ir = rr!(ascii::str_to_raw(&input), false);
        let out = rr!((padding::Pkcs7.pad_fn)(&ir, bs), false);
        let outs = rr!(ascii::raw_to_str(&out), false);
        assert_eq!(outs, exp);
        true
    }

    assert!(test("ABCDE", 8, "ABCDE\x03\x03\x03"));
    assert!(test("ABCDEFG", 8, "ABCDEFG\x01"));
    assert!(test("ABCDEFG", 4, "ABCDEFG\x01"));
}
