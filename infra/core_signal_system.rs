use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use std::any::Any;

type EventHandler = Box<dyn Fn(&dyn Any) + Send + Sync>;

pub struct EventBus {
    handlers: RwLock<HashMap<String, Vec<EventHandler>>>,
}

impl EventBus {
    pub fn new() -> Arc<Self> {
        Arc::new(EventBus {
            handlers: RwLock::new(HashMap::new()),
        })
    }
    pub fn subscribe(&self, event: &str, handler: EventHandler) {
        self.handlers.write().unwrap_or_else(|p| p.into_inner())
            .entry(event.to_string())
            .or_default()
            .push(handler);
    }
    pub fn emit(&self, event: &str, payload: &dyn Any) {
        if let Some(handlers) = self.handlers.read().unwrap_or_else(|p| p.into_inner()).get(event) {
            for handler in handlers {
                handler(payload);
            }
        }
    }
}
