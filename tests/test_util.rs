extern crate matasano;
use self::matasano::common::util;


#[test]
fn test_min_index() {
    let v = vec![12, 2, 5, 56, 3];
    assert_eq!(util::min_index(&v), Some(1));

    let v = vec![0.12, 0.02, 0.5, 0.00056, 3.1];
    assert_eq!(util::min_index(&v), Some(3));
}
