

use matasano::common::cipher::aes;
use matasano::common::ascii;


fn enc(plain: &str, key: &str, olen: usize) -> bool {
    let pr = mr!(ascii::str_to_raw(&plain), false);
    let kr = mr!(ascii::str_to_raw(&key), false);
    let cr = mr!(aes::encrypt(&pr, &kr, &aes::cbc_128_pkcs7), false);
    assert!(cr != pr);
    assert_eq!(cr.len(), olen);
    println!("{}", mr!(ascii::raw_to_str(&cr), false));
    true
}


#[test]
fn test_encrypt_cbc_128_pkcs7() {
        assert!(enc("this is test message of length 33..asdas", "YELLOW SUBMARINE", 48));
}
