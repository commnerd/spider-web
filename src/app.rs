use crate::server::Server;
use crate::config as config_crate;

use config::Config;

#[derive(Clone)]
pub struct App {
    pub config: Config,
    server: Option<Box<Server>>,
}

impl App {
    pub fn new() -> App {
        App{
            config: config_crate::load(),
            server: None
        }
    }

    pub fn run(mut self) {
        self.bootstrap().exec();
    }

    fn bootstrap(self) -> Self {
        App {
            config: self.config.clone(),
            server: Some(Box::new(Server::new(self))),
        }
    }

    fn exec(self) {
        let svr = self.server.clone();

        match svr {
            Some(server) => {
                server.run()
            },
            None => {
                println!("Woah!  Something went wrong!");
            },
        }
    }
}