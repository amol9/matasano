

//connection between two nodes on the network
pub struct Connection {
    receiver:   Receiver,
    buffer:     Vec<u8>
}


impl Connection {
    fn new(node: &Receiver) -> Connection {
        Connection {
            receiver: node
        }
    }

    
    fn send(&self, data: &Vec<u8>) -> Response {
        self.buffer = data.clone();
        self.receiver.receive(&data)
    }


    fn sniff(&self) -> Option<&Vec<u8>> {
        Ok(&self.data)
    }
}


