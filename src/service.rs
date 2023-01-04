// use std::any::type_name;
use std::marker::{Sync, Send};
pub trait Service: Sync + Send {
    fn type_name(&self) -> &'static str;
    fn run(&self);
}

pub struct ExampleService {}

impl Service for ExampleService {
    fn type_name(&self) -> &'static str {
        "ExampleService"
    }

    fn run(&self) {
        print!("Hello world!");
    }
}