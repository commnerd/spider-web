use crate::connection::Connection;
use crate::web;
use config::Config;
use std::io::prelude::*;
use std::net::TcpStream;
use std::process::Command;

#[derive(Clone)]
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

    pub fn run(&self) {
        let config = &self.config;

        self.runWebServer();
        
        println!("Shutting down.");
    }

    fn runContainerd(&self) {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                    .args(["/C", "dockerd", "--config", "./config/containerd.win"])
                    .output()
                    .expect("failed to execute process")
        } else {
            Command::new("sh")
                    .args(["/C", "dockerd", "--config", "./config/containerd.lnx"])
                    .output()
                    .expect("failed to execute process")
        };
    }
    
    fn runWebServer(&self) {
        web::serve();
    }
}