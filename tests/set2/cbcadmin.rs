
use matasano::set2::cbcadmin;


#[test]
fn test_cryptopals_case() {
    let authbox = m!(cbcadmin::init_authbox());
    assert!(m!(cbcadmin::auth_as_admin(&authbox)));
}

