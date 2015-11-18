
use matasano::set2::cbcadmin;


#[test]
fn test_cryptopals_case() {
    let authbox = r!(cbcadmin::init_authbox());
    assert!(r!(cbcadmin::auth_as_admin(&authbox)));
}

