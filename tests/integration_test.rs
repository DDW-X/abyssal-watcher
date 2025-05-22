
use analyzer::ml_analyzer::analyze_behavior;
use defense::ze_mode::ZEProtector;

#[test]
fn test_ml_analysis() {
    let malicious = "shellcode xor_loop injection";
    let benign = "hello world";
    assert!(analyze_behavior(malicious));
    assert!(!analyze_behavior(benign));
}

#[test]
fn test_ze_mode_scan() {
    ZEProtector::activate();
    let result = ZEProtector::inspect("fileless_malware injected");
    assert!(result);
}
