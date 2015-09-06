

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


#[derive(Show)]
enum Role {
    user,
    admin
}


impl FromStr for Role {
    fn from_str(s: &str) -> Result<Self, err::Error> {
        match s {
            "user"  => Self::user,
            "admin" => Self::admin,
            _       => mkerr!(format!("not a valid role: {}", s))
        }
    }
}


struct User {
    email:  String,
    uid:    u32,
    role:   Role
}


impl User {
    fn new(email: &str, uid: u32, role: &Role) -> User {
        User {
            email:  email.clone(),
            uid:    uid,
            role:   role
        }
    }


    fn encode(&self) -> Result<String, err::Error> {
        Ok(try!(url::encode(&vec![
            ("email", &self.email),
            ("uid", self.uid),
            ("role", &format!("{:?}", self)])))
    }


    fn decode(param_string: &str) -> Result<User, err::Error> {
        let params = try!(url::decode(&param_string));
        Ok(User {
            email:  params[0][1],
            uid:    params[1][1].parse::<u32>(),
            role:   
    }
}

