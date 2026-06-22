use std::collections::HashMap;

pub struct EventBus {
    handlers: HashMap<String, Vec<Box<dyn Fn()>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self { handlers: HashMap::new() }
    }
    
    pub fn subscribe<F>(&mut self, _handler: F) 
    where F: Fn() + 'static {
    }
    
    pub fn publish(&self, _event: impl std::fmt::Debug) {
    }
}
