
use matasano::common::cipher::padding;


macro_rules! padpkcs7 {
    ( $x: expr, $bs: expr, $exp: expr ) => ({ 
        let mut text = s!($x);
        m!(padding::pkcs7(&mut text, $bs));
        assert_eq!(text, $exp);
    });
}


#[test]
fn test_cryptopals_case() {
    let mut text = s!("YELLOW SUBMARINE");
    m!(padding::pkcs7(&mut text, 20));

    padding::print_pkcs7(&text, 20);
    assert_eq!(text, "YELLOW SUBMARINE\x04\x04\x04\x04");
}


#[test]
fn test_more() {
    padpkcs7!("YELLOW SUBMARINE", 16, "YELLOW SUBMARINE");
    padpkcs7!("YELLOW SUBMARIN", 16, "YELLOW SUBMARIN\x01");
    padpkcs7!("YELLOW SUBMARI", 16, "YELLOW SUBMARI\x02\x02");
    padpkcs7!("YELLOW", 5, "YELLOW\x04\x04\x04\x04");
}
