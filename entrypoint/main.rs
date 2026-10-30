
use defense::ze_mode::ZEProtector;
use engine::threat_detector;
use analyzer::ml_analyzer;
use infra::secure_logger;

fn main() -> anyhow::Result<()> {
    if defense::anti_debug::is_debugger_present() {
        println!("[ALERT] Debugger detected. Exiting."); return Ok(());
    }
    ZEProtector::activate();
    let _ = secure_logger::log_secure("[BOOT] ZE_MODE initialized");

    let test_data = "memory_injection polymorphic xor_loop shellcode";
    if ZEProtector::inspect(test_data) 
        || threat_detector::detect_anomaly(test_data)
        || ml_analyzer::analyze_behavior(test_data) 
    {
        println!("[ALERT] Multi-layer threat detected.");
        let _ = secure_logger::log_secure("[ALERT] Threat blocked and logged.");
    } else {
        println!("[OK] System is clean.");
        let _ = secure_logger::log_secure("[OK] Scan completed successfully.");
    }
    Ok(())
}
