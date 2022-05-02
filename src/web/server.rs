use crate::app::App;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/processes")]
fn processes() -> &'static str {
    "This will list processes"
}

pub fn serve(app: &App) {
    rocket::ignite()
        .mount("/", routes![
            index,
            processes
        ])
        .launch();
}

