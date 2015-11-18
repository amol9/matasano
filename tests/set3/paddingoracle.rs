
use matasano::set3::paddingoracle as po;


#[test]
fn test_cryptopals_case() {
    let obox = r!(po::OBox::new());

    for _ in 0 .. 5 {
        assert!(obox.string_valid(r!(po::break_cbc(&obox).as_ref())));
    }
}

