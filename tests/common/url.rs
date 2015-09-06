
use matasano::common::url;


#[test]
fn test_encode() {
    assert_eq!(m!(url::encode(&vec![("a", "test"), ("b", "10"), ("c", "something")])), "a=test&b=10&c=something");
}


#[test]
fn test_decode() {
    assert_eq!(m!(url::decode("a=test&b=10&c=something")), vec![(s!("a"), s!("test")), (s!("b"), s!("10")), (s!("c"), s!("something"))]);
}


