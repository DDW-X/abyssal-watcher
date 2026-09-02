//! Watcher Core: Periodically checks system integrity with dynamic strategy.

use std::time::Instant;
use log::info;

/// Trait representing a strategy for system checking.
pub trait CheckStrategy {
    fn check(&self) -> bool;
}

/// Default checking strategy
pub struct DefaultCheck;

impl CheckStrategy for DefaultCheck {
    fn check(&self) -> bool {
        info!("Performing default system integrity check...");
        // Placeholder for detailed checks
        true
    }
}

/// Core system watcher with pluggable check strategy.
pub struct Watcher<T: CheckStrategy> {
    last_check: Instant,
    strategy: T,
}

impl<T: CheckStrategy> Watcher<T> {
    pub fn new(strategy: T) -> Self {
        Watcher {
            last_check: Instant::now(),
            strategy,
        }
    }

    pub fn monitor(&mut self) {
        if self.last_check.elapsed().as_secs() > 1 {
            self.strategy.check();
            self.last_check = Instant::now();
        }
    }
}
