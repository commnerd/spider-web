use crate::connection::Connection;

pub struct Server {
    upstream_connection: Connection,
    downstream_connections: std::vec::Vec<Connection>,
}

impl <'svr> Server {
    pub fn new() -> Server {
        Server{
            upstream_connection: Connection::new(),
            downstream_connections: vec![],
        }
    }
}