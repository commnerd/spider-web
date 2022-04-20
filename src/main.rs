mod app;
mod server;
mod connection;
mod pipe;
mod config;

use app::App;

fn main() {
    let mut app = App::new();

    app.run();
}
