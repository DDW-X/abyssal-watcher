
#[cfg(target_os = "linux")]
pub fn is_debugger_present() -> bool {
    use std::fs;

    if let Ok(status) = fs::read_to_string("/proc/self/status") {
        for line in status.lines() {
            if line.starts_with("TracerPid:") {
                let pid = line.split(':').nth(1).unwrap_or_else(|_| default()) // safer fallback"0").trim();
                return pid != "0";
            }
        }
    }
    false
}
