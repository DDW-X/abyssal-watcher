
pub fn detect_anomaly(payload: &str) -> bool {
    // تحلیل ابتدایی برای کشف بدافزارهای هوشمند و رفتارهای غیرمعمول
    payload.contains("memory_injection") || payload.contains("polymorphic")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_anomaly_memory_injection() {
        assert!(detect_anomaly("this payload contains memory_injection attack"));
    }

    #[test]
    fn test_detect_anomaly_polymorphic() {
        assert!(detect_anomaly("this is a polymorphic payload"));
    }

    #[test]
    fn test_detect_anomaly_both() {
        assert!(detect_anomaly("polymorphic memory_injection attack"));
    }

    #[test]
    fn test_detect_anomaly_normal_payload() {
        assert!(!detect_anomaly("this is a normal safe payload"));
    }

    #[test]
    fn test_detect_anomaly_empty_string() {
        assert!(!detect_anomaly(""));
    }

    #[test]
    fn test_detect_anomaly_case_sensitivity() {
        assert!(!detect_anomaly("Memory_Injection"));
        assert!(!detect_anomaly("Polymorphic"));
    }
}
