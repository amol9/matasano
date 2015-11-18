
use matasano::common::cipher::padding;
use matasano::common::ascii;


macro_rules! padpkcs7 {
    ( $x: expr, $bs: expr, $exp: expr ) => ({ 
        let text = $x;
        let ptext = rts!(&r!(padding::pkcs7_pad(&raw!(&text), $bs)));
        assert_eq!(ptext, $exp);
    });
}


#[test]
fn test_cryptopals_case() {
    let text = "YELLOW SUBMARINE";
    let ptext = rts!(&r!(padding::pkcs7_pad(&raw!(&text), 20)));

    r!(padding::print_pkcs7(&ptext, 20));
    assert_eq!(ptext, "YELLOW SUBMARINE\x04\x04\x04\x04");
}


#[test]
fn test_more() {
    padpkcs7!("YELLOW SUBMARINE", 16, "YELLOW SUBMARINE");
    padpkcs7!("YELLOW SUBMARIN", 16, "YELLOW SUBMARIN\x01");
    padpkcs7!("YELLOW SUBMARI", 16, "YELLOW SUBMARI\x02\x02");
    padpkcs7!("YELLOW", 5, "YELLOW\x04\x04\x04\x04");
}
