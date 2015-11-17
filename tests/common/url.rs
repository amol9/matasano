
use matasano::common::url;


#[test]
fn test_encode() {
    assert_eq!(m!(url::encode(&vec![("a", "test"), ("b", "10"), ("c", "something")])), "a=test&b=10&c=something");
}

#[test]
fn test_decode() {
    let result = m!(url::decode("a=test&b=10&c=something"));
    let exp = vec![("a", "test"), ("b", "10"), ("c", "something")];

    assert_eq!(result.len(), exp.len());
    for i in 0 .. result.len() {
        assert_eq!(result[i].0, exp[i].0);
        assert_eq!(result[i].1, exp[i].1);
    }
}

