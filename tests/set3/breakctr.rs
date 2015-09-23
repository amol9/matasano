
use matasano::set3::breakctr as bc;
use matasano::common::ascii;


macro_rules! raw {
    ( $x : expr ) => ( m!( ascii::str_to_raw( $x ) ) );
}


#[test]
fn test_detect_trigrams() {
    let input: Vec<Vec<u8>> = vec![raw!("abcfeoowoptpeefs"),
                                raw!("abcrklsriwewfks"),
                                raw!("ab"),
                                raw!("dgrdkls")];

    let input_max_len: usize = *input.iter().map(|i| i.len()).collect::<Vec<usize>>().iter().max().unwrap();

    let (result_i, result_c) = bc::detect_trigrams(&input);

    assert_eq!(result_i.len(), input_max_len);
    assert!(result_i.starts_with(&[0, 1, 2, 3, 0, 1, 2]));
    assert!(result_c.starts_with(&raw!("abc\x00kls")));
}
