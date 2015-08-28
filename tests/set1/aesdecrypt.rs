extern crate matasano;
use self::matasano::set1::aesdecrypt as ad;


#[test]
fn test_cryptopals_case() {
    let plain = m!(ad::decrypt_from_file("data/7.txt", "YELLOW SUBMARINE"));
    let expected_firstl: &str = "I'm back and I'm ringin' the bell";

    assert!(plain.starts_with(expected_firstl));
    println!("{}", plain);
}
