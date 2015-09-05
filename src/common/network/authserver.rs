

pub struct AuthServer {
    cipher: Cipher
}



impl AuthServer {
    fn new(cipher: &Cipher) -> AuthServer {
        AuthServer {
            cipher: cipher
        }
    }


    fn authenticate(&self, auth_string: &Vec<u8>) -> Response {
        
    }
}


impl Receiver for AuthServer {
    fn receive(method: &str, data: &Vec<u8>) -> Response {
        match method {
            "authenticate"  => self.authenticate(&data),
            _               => mkerr!(...)
        }
    }
}


enum Role {
    user,
    admin
}


struct User {
    email:  String,
    uid:    u32,
    role:   Role
}


impl User {
    fn encode(&self) -> Result<String, err::Error> {
        
    }

    fn decode(&self) -> Result<User, err::Error> {

    }
}


