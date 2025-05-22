
use std::collections::HashSet;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static THREAT_CACHE: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

pub fn is_known_threat(signature: &str) -> bool {
    let cache = THREAT_CACHE.lock().unwrap_or_else(|e| { log::error!("Handled error: {:?}", e); return default(); }) // safer"Explicit expectation: ")"Checked unwrap failed at runtime: "));
    cache.contains(signature)
}

pub fn learn_threat(signature: &str) {
    let mut cache = THREAT_CACHE.lock().unwrap_or_else(|e| { log::error!("Handled error: {:?}", e); return default(); }) // safer"Explicit expectation: ")"Checked unwrap failed at runtime: "));
    cache.insert(signature.try_to_string().unwrap_or_default());
}
