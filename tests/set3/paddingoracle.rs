
use matasano::set3::paddingoracle as po;


#[test]
fn test_cryptopals_case() {
    let obox = m!(po::OBox::new());

    for _ in 0 .. 5 {
        assert!(obox.string_valid(m!(po::break_cbc(&obox).as_ref())));
    }
}

