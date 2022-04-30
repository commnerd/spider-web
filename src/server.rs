use crate::app::App;
use crate::connection::Connection;
use crate::web;
use std::process::Command;

#[derive(Clone)]
pub struct Server {
    app: App,
    connections: Vec<Connection>,
}

impl Server {
    pub fn new(app: App) -> Self {
        Server{
            app: app,
            connections: vec![],
        }
    }

    pub fn run(&self) {
        let config = &self.app.config;

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