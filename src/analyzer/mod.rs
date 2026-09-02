use std::collections::HashMap;

#[derive(Debug)]
pub struct Signature {
    pub id: String,
    pub description: String,
    pub category: String,
    pub severity: u8, // 1 - 10
}

pub struct ThreatAnalyzer {
    signatures: HashMap<String, Signature>,
}

impl ThreatAnalyzer {
    pub fn new() -> Self {
        let mut signatures = HashMap::new();
        signatures.insert("unusual_port_usage".into(), Signature {
            id: "unusual_port_usage".into(),
            description: "Unusual port activity".into(),
            category: "network".into(),
            severity: 6,
        });
        signatures.insert("code_injection_detected".into(), Signature {
            id: "code_injection_detected".into(),
            description: "Possible code injection".into(),
            category: "memory".into(),
            severity: 9,
        });
        Self { signatures }
    }

    pub fn analyze(&self, event: &str) -> Option<&Signature> {
        self.signatures.get(event)
    }

    pub fn score(&self, event: &str) -> u32 {
        if let Some(sig) = self.signatures.get(event) {
            sig.severity as u32 * 10
        } else {
            0
        }
    }
}

pub mod ml_analyzer;
pub mod threat_cache;
