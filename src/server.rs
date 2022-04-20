use crate::connection::Connection;
use crate::config::Config;

#[derive(Default)]
pub struct Server {
    upstream_connection: Connection,
    downstream_connections: std::vec::Vec<Connection>,
}

impl <'svr> Server {
    pub fn new(conf: &Config) -> Server {
        Server{
            upstream_connection: Connection::new(),
            downstream_connections: vec![],
        }
    }
}