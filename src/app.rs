use crate::server::Server;

pub struct App {
    server: Server,
}

impl App {
    pub fn new() -> App {
        App{ server: Server::new() }
    }

    pub fn bootstrap(&self) {

    }

    pub fn run(&self) {
        println!("Hello World!");
    }

    pub fn teardown(&self) {
        
    }
}