// use std::any::type_name;

pub trait Service {
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