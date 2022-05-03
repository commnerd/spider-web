use crate::app::App;
use crate::connection::Connection;
use crate::containerd;
use crate::web;
use std::process::Command;
use std::thread;

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

        self.runContainerd();
        self.runWebServer();
        
        println!("Shutting down.");
    }

    fn runContainerd(&self) {
        thread::spawn(|| {
            containerd::client::connect();
        });
    }
    
    fn runWebServer(&self) {
        web::serve(&self.app);
    }
}