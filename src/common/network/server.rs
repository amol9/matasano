
use common::err;


pub struct Server {
    pub name:           String,
    cipher:             Cipher,
    suffix:             Vec<u8>,
    authserver_conn:    Connection,
}


impl Server {
    fn new(name: &str) -> Self {
        Server {
            name:   name
        }
    }

    fn profile_for()

    fn suffix()
}


impl Receiver for Server {
    fn receive(&self, method: &str, data: &Vec<u8>) -> Response {
        match method {
            "suffix"        => self.suffix(&data),
            "profile_for"   => self.profile_for(&data),
            _               => err
        }
    }
}

