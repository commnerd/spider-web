mod app;
mod config;
mod connection;
mod pipe;
mod server;
mod thread;

use app::App;

fn main() {
    let app = App::new();

    app.run();
}
