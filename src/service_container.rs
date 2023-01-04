use std::collections::HashMap;
use std::thread;
use std::sync::Arc;

use crate::service::Service;

pub struct ServiceContainer<T: Service + 'static> {
    pub service_count: u8,
    pub services: Box<HashMap<&'static str, Arc<T>>>,
}

impl<T: Service + 'static> ServiceContainer<T> {
    pub fn new() -> Self {
        ServiceContainer{
            service_count: 0,
            services: Box::new(HashMap::new()),
        }
    }

    pub fn run(mut self) {
        for (_, inst) in self.services.iter() {
            let inst_clone = inst.clone();
            let handle = thread::spawn(move || {
                inst_clone.run();
                self.service_count -= 1;
            });
            handle.join().unwrap();
        }
    }

    pub fn register(mut self, service: T) -> Self {
        self.service_count += 1;

        self.services.insert(service.type_name(), Arc::<T>::new(service));

        self
    }
}