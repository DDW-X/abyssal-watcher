use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::any::Any;

type EventHandler = Box<dyn Fn(&dyn Any) + Send + Sync>;

pub struct EventBus {
    handlers: Mutex<HashMap<String, Vec<EventHandler>>>,
}

impl EventBus {
    pub fn new() -> Arc<Self> {
        Arc::new(EventBus {
            handlers: Mutex::new(HashMap::new()),
        })
    }

    pub fn subscribe(&self, event: &str, handler: EventHandler) {
        self.handlers.lock().unwrap()
            .entry(event.to_string())
            .or_default()
            .push(handler);
    }

    pub fn emit(&self, event: &str, payload: &dyn Any) {
        if let Some(handlers) = self.handlers.lock().unwrap().get(event) {
            for handler in handlers {
                handler(payload);
            }
        }
    }
}
