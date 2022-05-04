use config::Config;
use crate::connection::Connection;
use crate::containerd;
use crate::web;
use std::process::Command;
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
        self.runContainerd();
        self.runWebServer();
        
        println!("Shutting down.");
    }

    fn runContainerd(&self) {
        let config = self.config.clone();
        thread::spawn(|| {
            containerd::serve(config);
        });
    }
    
    fn runWebServer(&self) {
        web::serve(self.config.clone());
    }
}