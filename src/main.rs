
mod service_container;
mod service;

use service_container::ServiceContainer;
use service::ExampleService;

fn main() {
    let service_container = ServiceContainer::new();
    service_container
        .register(ExampleService{})
        .run();
}
