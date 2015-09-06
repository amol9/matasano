

pub struct Network {
    nodes:          Vec<Receiver>,
    connections:    Vec<Connection>
}


impl Network {
    fn new() -> Network {
        nodes = Vec::<Receiver>::new();
        connections = Vec::<Connection>::new();
    }


    fn connect(src: &str, dest: &str) -> Result<&Connection, err::Error> {
        match self.connections.find(|&c| c.get_src_dest() == (src, dest)) {
            Some(v) => return conn,
            None    => {}
        };

        fn find_node(name: &str) -> Result<Receiver, err::Error> {
            match self.nodes.find(|&x| x == name) {
                Some(v) => v,
                None    => return mkerr!(format!("cannot find node: {}", name))
            }
        }

        let src_node = try!(find_node(&src));
        let dest_node = try!(find_node(&dest));

        let conn = Connection::new(&src_node, &dest_node);
        connections.push(conn);

        Ok(&conn)
    }


    fn add_node(node: &Receiver) {
        nodes.push(node); 
    }

}


pub fn get_network -> Result<Network, err::Error> {
    let mut network = Network::new();

    let server = Server::new(&cipher, "server");
    network.add_node(&server);

    let server2 = Server::new(&cipher, "server2");
    network.add_node(&server2);

    let authserver = AuthServer::new(&cipher, "authserver");
    network.add_node(&authserver);

    Ok(network)
}


