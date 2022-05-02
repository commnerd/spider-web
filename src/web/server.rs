use crate::app::App;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn serve(app: &App) {
    rocket::ignite().mount("/", routes![index]).launch();
}

