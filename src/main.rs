#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod app;
mod config;
mod connection;
mod pipe;
mod server;
mod web;

use app::App;

fn main() {
    let app = App::new();

    app.run();
}
