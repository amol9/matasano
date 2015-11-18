
use matasano::set3::ctr;


#[test]
fn test_cryptopals_case() {
    assert_eq!(r!(ctr::ctr_crypt("L77na/nrFsKvynd6HzOoG7GHTLXsTVu9qvY/2syLXzhPweyyMTJULu/6/kXX0KSvoOLSFQ==", "YELLOW SUBMARINE")),
        "Yo, VIP Let's kick it Ice, Ice, baby Ice, Ice, baby ");
}

