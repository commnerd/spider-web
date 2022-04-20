use crate::server::Server;
use crate::config::Config;

#[derive(Default)]
pub struct App {
    config: Config,
    server: Server,
}

impl App {
    pub fn new() -> App {
        App::default()
    }

    pub fn run(&mut self) {
        self.bootstrap();
        self.exec();
        self.teardown();
    }

    fn bootstrap(&mut self) {
        self.config = Config::init();
        self.server = Server::new(&self.config);
    }

    fn exec(&self) {

    }

    fn teardown(&self) {
        
    }
}