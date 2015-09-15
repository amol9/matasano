
use matasano::set3::paddingoracle as po;


#[test]
fn test_cryptopals_case() {
    let obox = po::OBox::new();

    for _ in 0 .. 3 {
        assert!(obox.string_valid(m!(po::break_cbc(&obox)));
    }
}
