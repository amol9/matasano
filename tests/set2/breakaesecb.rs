
use matasano::set2::breakaesecb as bae;
use matasano::common::{base64, ascii};
use matasano::common::cipher::cipherbox as cb;


#[test]
fn test_cryptopals_case() {
    let plainb64 = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg\
                    aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq\
                    dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg\
                    YnkK";

    let cbox = r!(cb::init(&plainb64));
    let plain = r!(bae::break_aes_ecb(&cbox));

    assert_eq!(plain, r!(ascii::raw_to_str(&r!(base64::base64_to_raw(&plainb64)))));
}


#[test]
fn test_more() {
    fn test(plain: &str) {
        let raw = r!(ascii::str_to_raw(&plain));
        let b64 = r!(base64::raw_to_base64(&raw));
        let cbox = r!(cb::init(&b64));
        let p = r!(bae::break_aes_ecb(&cbox));
        assert_eq!(p, plain);
    }

    test("this is test message");
    test("a");
    //test(""); need to fix
    test("This is a longer message. Please bear with me.");
}
