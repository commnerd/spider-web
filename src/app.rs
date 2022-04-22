use crate::server::Server;
use crate::config as config_crate;

use config::Config;

pub struct App {
    config: Config,
    server: Option<Server>,
}

impl App {
    pub fn new() -> App {
        App{
            config: config_crate::load(),
            server: None
        }
    }

    pub fn run(&self) {
        self.bootstrap();
    }

    fn bootstrap(&self) -> Self {
        App {
            config: self.config.clone(),
            server: Some(Server::new(self.config.clone())),
        }
    }

    fn exec(&'static self) {
    }
}