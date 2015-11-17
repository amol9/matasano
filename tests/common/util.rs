extern crate matasano;
use self::matasano::common::{util, ascii};


#[test]
fn test_min_index() {
    let v = vec![12, 2, 5, 56, 3];
    assert_eq!(util::min_index(&v), Some(1));

    let v = vec![0.12, 0.02, 0.5, 0.00056, 3.1];
    assert_eq!(util::min_index(&v), Some(3));
}


#[test]
fn test_min_indices() {
    let v = vec![12, 2, 5, 56, 3];
    assert_eq!(util::min_indices(&v, 2), Some(vec![1, 4]));
}


macro_rules! hammd {
    ( $x : expr, $y : expr )    => ( util::hamming_distance($x as u8, $y as u8); );
}


#[test]
fn test_hamming_distance() {
    assert_eq!(hammd!('a', 'g'), 2);
    assert_eq!(hammd!('a', 'x'), 3);
}


macro_rules! hammv {
    ( $x : expr, $y : expr ) => ( m!(
            util::hamm_vec(&m!(ascii::str_to_raw($x)), &m!(ascii::str_to_raw($y)))) );
}


#[test]
fn test_hamm_vec() {
    assert_eq!(hammv!("this is a test", "wokka wokka!!!"), 37);
}


#[test]
fn test_transpose_vec() {
    let input = vec![1, 2, 3, 4, 1, 2, 3, 4, 1, 2];
    assert_eq!(m!(util::transpose_vec(&input, 4)), vec![vec![1, 1, 1], vec![2, 2, 2], vec![3, 3], vec![4, 4]]);
}


#[test]
fn test_transpose_str() {
    let input = "abcdabcdabcdabcdabc";
    assert_eq!(m!(util::transpose_str(&input, 4)), vec!["aaaaa", "bbbbb", "ccccc", "dddd"]);
}


#[test]
fn test_dup() {
    assert_eq!(util::dup::<u32>(&vec![1, 2, 1, 3, 4]), vec![(1, 2)]);
    assert_eq!(util::dup::<u32>(&vec![1, 2, 1, 3, 4]), vec![(1, 2)]);
    assert_eq!(util::dup::<u32>(&vec![1, 2, 1, 3, 4, 2, 2]), vec![(1, 2), (2, 3)]);
}

#[test]
fn test_freq() {
    assert_eq!(util::freq::<u32>(&vec![1, 2, 1, 3, 4]), vec![(1, 2), (2, 1), (3, 1), (4, 1)]);
    assert_eq!(util::freq::<u32>(&vec![1, 2, 1, 3, 4, 2, 2]), vec![(1, 2), (2, 3), (3, 1), (4, 1)]);
}

