
use common::err;


pub struct Server {
    pub name:           String,
    cipher:             Cipher,
    suffix:             Vec<u8>,
    network:            &Network
}


impl Server {
    fn new(name: &str, cipher: &Cipher) -> Self {
        Server {
            name:   name
        }
    }

    fn set_suffix(&mut self, data: &Vec<u8>) {
        self.suffix = data.clone();
    }

    fn profile_for(&self, email: &Vec<u8>) -> Response {
        let email_str = ertry!(ascii::raw_to_str(&email));

        let user = User::new(&email, 10, "user");
        let encoded = ertry!(user.encode());
        let enc_raw = ertry!(ascii::str_to_raw(&encoded));
        ertry!(self.cipher.encrypt(&enc_raw))
    }

    fn suffix(&self, data: &Vec<u8>) -> Response {
        let mut final_data = data.clone();
        final_data.extend(&self.suffix);
        let cipherdata = ertry!(self.cipher.encrypt(&final_data));
    
        let conn = self.network.connect(&self.name, "server2");
        conn.send(&cipherdata)
    }
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

