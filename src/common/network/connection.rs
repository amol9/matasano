

//connection between two nodes on the network
pub struct Connection {
    src:        &Receiver,
    dest:       &Receiver,
    buffer:     Vec<u8>
}


impl Connection {
    fn new(src: &receiver, dest: &Receiver) -> Connection {
        Connection {
            src:    src,
            dest:   dest,
            buffer: Vec::<u8>>::new()
        }
    }

    
    fn send(&mut self, data: &Vec<u8>) -> Response {
        self.buffer = data.clone();
        self.dest.receive(&data)
    }


    fn sniff(&self) -> Option<&Vec<u8>> {
        Ok(&self.data)
    }

    fn get_src_dest(&self) -> (&str, &str) {
        (self.src.name, self.dest.name)
    }
}

