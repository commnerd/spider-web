mod app;
mod server;
mod connection;
mod pipe;

use app::App;

fn main() {
    let app = App::new();

    app.bootstrap();
    app.run();
    app.teardown();
}
