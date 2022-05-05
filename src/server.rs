use config::Config;
use crate::connection::Connection;
use crate::containerd;
use crate::web;
use std::thread;

#[derive(Clone)]
pub struct Server {
    config: Config,
    connections: Vec<Connection>,
}

impl Server {
    pub fn new(config: Config) -> Self {
        Server{
            config: config,
            connections: vec![],
        }
    }

    pub fn run(&self) {
        self.run_containerd();
        self.run_web_server();
        
        println!("Shutting down.");
    }

    fn run_containerd(&self) {
        let config = self.config.clone();
        thread::spawn(|| {
            containerd::serve(config);
        });
    }
    
    fn run_web_server(&self) {
        web::serve(self.config.clone());
    }
}