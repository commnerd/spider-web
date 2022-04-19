struct App {}

impl App {
    fn run(&self) {
        println!("Hello World!")
    }
}

fn main() {
    let app = App{};

    app.run();
}
