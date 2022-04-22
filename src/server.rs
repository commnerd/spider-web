use crate::connection::Connection;
use config::Config;

pub struct Server {
    config: Config,
    connections: Vec<Connection>,
}

impl Server {
    pub fn new(conf: Config) -> Self {
        Server{
            config: conf,
            connections: vec![],
        }
    }
}