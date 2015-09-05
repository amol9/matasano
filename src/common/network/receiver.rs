
use common::err;


trait Receiver {
    fn receive(&self, method: &str, data: &Vec<u8>) -> Response;
}


pub enum Response {
    Role,
    Vec<u8>,
    err::Error
}


pub struct Request {
    method: &str,
    data:   &Vec<u8>
}

