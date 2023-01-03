use std::collections::HashMap;
// use std::thread;

use crate::service::Service;

pub struct ServiceContainer {
    pub service_count: u8,
    pub services: Box<HashMap<&'static str, Box<dyn Service>>>,
}

impl ServiceContainer {
    pub fn new() -> Self {
        ServiceContainer{
            service_count: 0,
            services: Box::new(HashMap::new()),
        }
    }

    pub fn run(mut self) {
        for (_, inst) in self.services.iter() {
            inst.run();
            self.service_count -= 1;
        }
    }

    pub fn register<T: Service + 'static>(mut self, service: T) -> Self {
        self.service_count += 1;

        self.services.insert(service.type_name(), Box::<T>::new(service));

        self
    }
}