
use std::collections::HashSet;
use std::sync::RwLock;
use once_cell::sync::Lazy;

static THREAT_CACHE: Lazy<RwLock<HashSet<String>>> = Lazy::new(|| RwLock::new(HashSet::new()));

pub fn is_known_threat(signature: &str) -> bool {
    let cache = THREAT_CACHE.read().unwrap_or_else(|p| p.into_inner());
    cache.contains(signature)
}

pub fn learn_threat(signature: &str) {
    let mut cache = THREAT_CACHE.write().unwrap_or_else(|p| p.into_inner());
    cache.insert(signature.to_string());
}
