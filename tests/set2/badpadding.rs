
use matasano::common::cipher::padding;
use matasano::common::ascii;


#[test]
fn test_cryptopals_case() {
    assert_eq!(m!(padding::pkcs7_detect(&r!("ICE ICE BABY\x04\x04\x04\x04"), 16)), 4);
    assert!(me!(padding::pkcs7_detect(&r!("ICE ICE BABY\x05\x05\x05\x05"), 16)));
    assert!(me!(padding::pkcs7_detect(&r!("ICE ICE BABY\x01\x02\x03\x04"), 16)));
}

