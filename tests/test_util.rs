extern crate matasano;
use self::matasano::common::{util, ascii};


#[test]
fn test_min_index() {
    let v = vec![12, 2, 5, 56, 3];
    assert_eq!(util::min_index(&v), Some(1));

    let v = vec![0.12, 0.02, 0.5, 0.00056, 3.1];
    assert_eq!(util::min_index(&v), Some(3));
}


macro_rules! hammd {
    ( $x : expr, $y : expr )    => ( util::hamming_distance($x as u8, $y as u8); );
}


#[test]
fn test_hamming_distance() {
    assert_eq!(hammd!('a', 'g'), 2);
    assert_eq!(hammd!('a', 'x'), 3);
}


macro_rules! s {
	( $x : expr ) => ( String::from($x) );
}

macro_rules! match_res {
    ( $x : expr ) => ( match $x {
        Ok(v)   => v,
        Err(e)  => { println!("{}", e); assert!(false); return; }
    } );
}


macro_rules! hammv {
    ( $x : expr, $y : expr ) => ( match_res!(
            util::hamm_vec(&match_res!(ascii::str_to_raw(&s!($x))), &match_res!(ascii::str_to_raw(&s!($y))))) );
}


#[test]
fn test_hamm_vec() {
    assert_eq!(hammv!("this is a test", "wokka wokka!!!"), 37);
}


#[test]
fn test_transpose_vec() {
    let input = vec![1, 2, 3, 4, 1, 2, 3, 4, 1, 2];
    assert_eq!(match_res!(util::transpose_vec(&input, 4)), vec![vec![1, 1, 1], vec![2, 2, 2], vec![3, 3], vec![4, 4]]);
}

