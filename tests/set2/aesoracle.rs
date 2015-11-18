
use matasano::set2::aesoracle;


#[test]
fn test_cryptopals_case() {
    let (success, failure) = r!(aesoracle::detect_aes_mode(100));
    assert_eq!((success, failure), (100, 0));
}

