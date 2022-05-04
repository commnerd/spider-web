use crate::server::Server;
use crate::config as config_crate;

use config::Config;

#[derive(Clone)]
pub struct App {
    pub config: Config,
    server: Option<Server>,
}

impl App {
    pub fn new() -> App {
        App{
            config: config_crate::load(),
            server: None
        }
    }

    pub fn run(self) {
        let app = self.bootstrap();

        app.exec();
    }

    fn bootstrap(self) -> Self {
        let config = &self.config;

        App{
            config: config.clone(),
            server: Some(Server::new(config.clone()))
        }
    }

    fn exec(self) {
        match self.server {
            Some(server) => {
                server.run();
            },
            None => {
                println!("Woah!  Something went wrong!");
            },
        }
    }
}