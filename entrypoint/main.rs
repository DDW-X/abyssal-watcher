
use defense::ze_mode::ZEProtector;
use engine::threat_detector;
use analyzer::ml_analyzer;
use infra::secure_logger;

fn main() {
    if defense::anti_debug::is_debugger_present() {
        println!("[ALERT] Debugger detected. Exiting."); return;
    }
    ZEProtector::activate();
    if let Err(e) = secure_logger::log_secure("[BOOT] ZE_MODE initialized") {
        eprintln!("[ERROR] Failed to securely log boot event: {:?}", e);
    }

    let test_data = "memory_injection polymorphic xor_loop shellcode";
    if ZEProtector::inspect(test_data) 
        || threat_detector::detect_anomaly(test_data)
        || ml_analyzer::analyze_behavior(test_data) 
    {
        println!("[ALERT] Multi-layer threat detected.");
        if let Err(e) = secure_logger::log_secure("[ALERT] Threat blocked and logged.") {
            eprintln!("[ERROR] Failed to securely log alert: {:?}", e);
        }
    } else {
        println!("[OK] System is clean.");
        if let Err(e) = secure_logger::log_secure("[OK] Scan completed successfully.") {
            eprintln!("[ERROR] Failed to securely log scan success: {:?}", e);
        }
    }
}
