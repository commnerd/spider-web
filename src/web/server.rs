use config::Config;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/processes")]
fn processes() -> &'static str {
    "This will list processes"
}

pub fn serve(config: Config) {
    rocket::ignite()
        .manage(config)
        .mount("/", routes![
            index,
            processes
        ])
        .launch();
}

