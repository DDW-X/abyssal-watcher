use syslog::{Facility, Formatter3164};
use log::{info, warn};

pub fn init_syslog() {
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "abyssal_watcher".into(),
        pid: 0,
    };

    match syslog::unix(formatter) {
        Ok(logger) => {
            let _ = log::set_boxed_logger(Box::new(logger))
                .map(|()| log::set_max_level(log::LevelFilter::Info));
        }
        Err(e) => {
            eprintln!("Unable to connect to syslog: {}", e);
        }
    }
}

pub fn log_threat(signature: &str) {
    info!("Threat detected: {}", signature);
}

pub fn log_warning(msg: &str) {
    warn!("{}", msg);
}
