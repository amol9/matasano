
use matasano::set3::breakctr as bc;
use matasano::set3::breakctr2 as bc2;
use matasano::common::{ascii, util, base64};


#[test]
fn test_cryptopals_case_auto() {
    let filepath = "data/20.txt";
    let input = m!(util::read_file_to_str(&filepath));

    let ciphers = m!(bc::generate_ciphers_from_file(&filepath));
    let keystream = m!(bc2::break_ctr(&ciphers));
    let plains = bc::xor_keystream(&ciphers, &keystream);

    let mut failures: usize = 0;

    for (line, plain) in input.lines().zip(&plains) {
        let dline = m!(ascii::raw_to_str(&m!(base64::base64_to_raw(&line))));
        if dline != *plain {
            failures += dline.chars().zip(plain.chars()).filter(|&(l, p)| l != p).count();
        }
    }
    let failure_ratio = failures as f32 / input.len() as f32;

    assert!(failure_ratio < 0.01)       // known issue: problem with last few characters, so, decryption is apprx. 99% accurate
}

#[test]
fn test_cryptopals_case_manual() {
    let filepath = "data/20.txt";
    let input = m!(util::read_file_to_str(&filepath));

    let ciphers = m!(bc::generate_ciphers_from_file(&filepath));
    let guesses = vec![(28, "heart"), (21, "in peace"), (46, "t #########"),
                    (26, "whole #######"), (46, "money"), (26, "ry")];

    let plains = m!(bc2::break_ctr_with_manual_guess_for_last_chars(&ciphers, &guesses));

    for (line, plain) in input.lines().zip(&plains) {
        let dline = m!(ascii::raw_to_str(&m!(base64::base64_to_raw(&line))));
        assert_eq!(dline, *plain);
    }
}

