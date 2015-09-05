

pub struct Network {
    nodes:          Vec<Receiver>,
    connections:    Vec<Connection>
}


impl Network {
    fn new() -> Network {
        nodes = Vec::<Receiver>::new();
        connections = Vec::<Connection>::new();
    }


    fn connect(name: &str) -> Result<&Connection, err::Error> {
        let node = nodes.get(name);
        let conn = Connection::new(&node);
        connections.push(conn);
        Ok(&conn)
    }


    fn add_node(node: Receiver, name: &str) {
        nodes.push(server); 
    }

}

pub fn get_network13 -> {
    let network = Network::new();

    let server = Server::new(&cipher, "server");
    network.add(&server);

    let authserver = AuthServer::new(&cipher, "authserver");
    network.add(&authserver);

    Ok(network)
}


