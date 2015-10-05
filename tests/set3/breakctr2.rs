
use matasano::set3::breakctr as bc;
use matasano::set3::breakctr2 as bc2;
use matasano::common::{ascii, util, base64};


#[test]
fn test_cryptopals_case() {
    let filepath = "data/20.txt";
    let input = m!(util::read_file_to_str(&filepath));

    let ciphers = m!(bc::generate_ciphers_from_file(&filepath));

    let (plains, _) = m!(bc2::break_ctr(&ciphers));

    for (line, plain) in input.lines().zip(&plains) {
        let dline = m!(ascii::raw_to_str(&m!(base64::base64_to_raw(&line))));
        assert_eq!(dline, *plain);
    }
}

