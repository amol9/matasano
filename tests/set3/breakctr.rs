
use matasano::set3::breakctr as bc;
use matasano::common::{ascii, util, base64};


macro_rules! raw {
    ( $x : expr ) => ( m!( ascii::str_to_raw( $x ) ) );
}


#[test]
fn test_detect_trigrams() {
    let input: Vec<Vec<u8>> = vec![raw!("abcfeoowoptpeefs"),
                                raw!("abcrklsriwewfks"),
                                raw!("ab"),
                                raw!("dgrdkls"),
                                raw!("wer423jfsds"),
                                raw!("wert4gwew")];

    let input_max_len: usize = *input.iter().map(|i| i.len()).collect::<Vec<usize>>().iter().max().unwrap();

    let (result_i, result_c) = bc::detect_trigrams(&input);

    assert_eq!(result_i.len(), input_max_len);
    assert!(result_i.starts_with(&[0, 1, 2, 3, 0, 1, 2]));
    assert!(result_c.starts_with(&[raw!("aw"), raw!("be"), raw!("cr"), vec![], raw!("k"), raw!("l"), raw!("s")]));
}


#[test]
fn test_cryptopals_case_auto() {
    let filepath = "data/19.txt";
    let input = m!(util::read_file_to_str(&filepath));

    let ciphers = m!(bc::generate_ciphers_from_file(&filepath));
    let (plains, _) = m!(bc::break_ctr(&ciphers));

    let mut failures: usize = 0;

    for (line, plain) in input.lines().zip(&plains) {
        let dline = m!(ascii::raw_to_str(&m!(base64::base64_to_raw(&line))));
        if dline.to_lowercase() != *plain.to_lowercase() {
            failures += dline.chars().zip(plain.chars()).filter(|&(l, p)| l != p).count();
        }
    }
    let failure_ratio = failures as f32 / input.len() as f32;

    assert!(failure_ratio < 0.01)       // known issue: problem with last few characters, so, decryption is apprx. 99% accurate
}


#[test]
fn test_cryptopals_case_manual() {
    let filepath = "data/19.txt";
    let input = m!(util::read_file_to_str(&filepath));

    let ciphers = m!(bc::generate_ciphers_from_file(&filepath));
    let guesses = vec![(4, "head"), (37, "turn,")];

    let plains = m!(bc::break_ctr_with_manual_guess_for_last_chars(&ciphers, &guesses));

    for (line, plain) in input.lines().zip(&plains) {
        let dline = m!(ascii::raw_to_str(&m!(base64::base64_to_raw(&line))));
        assert_eq!(dline.to_lowercase(), plain.to_lowercase());
    }
}

