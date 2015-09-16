
use matasano::common::cipher::aes;
use matasano::common::ascii;


enum Op {
    encrypt,
    decrypt
}


fn encdec(input: &str, key: &str, olen: usize, op: Op) -> Option<String> {
    let ir = rn!(ascii::str_to_raw(&input));
    let kr = rn!(ascii::str_to_raw(&key));
    let or: Vec<u8>;

    match op {
        Op::encrypt => or = rn!(aes::encrypt(&ir, &kr, &aes::cbc_128_pkcs7)),
        Op::decrypt => or = rn!(aes::decrypt(&ir, &kr, &aes::cbc_128_pkcs7)),
    };

    assert!(or != ir);
    assert_eq!(or.len(), olen);
    println!("{}", rn!(ascii::raw_to_str(&or)));
    Some(rn!(ascii::raw_to_str(&or)))
}


fn test_enc(input: &str, key: &str, olen: usize) -> String {
    encdec(&input, &key, olen, Op::encrypt).unwrap()
}


fn test_dec(input: &str, key: &str, olen: usize) -> String {
    encdec(&input, &key, olen, Op::decrypt).unwrap()
}


fn test_enc_dec(plain: &str, key: &str) {
    let cipher: String;

    cipher = test_enc(&plain, &key, (plain.len() as f32 / 16f32).ceil() as usize * 16 + 16);
    test_dec(&cipher, &key, plain.len());
}


#[test]
fn test_cbc_128_pkcs7() {
    test_enc_dec("this is test message of length 33.", "YELLOW SUBMARINE");
    test_enc_dec("hello", "YELLOW SUBMARINE");
}
