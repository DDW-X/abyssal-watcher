mod api;
mod logs;
mod infra;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logs::init_syslog();
    println!("Starting Abyssal Watcher backend on 0.0.0.0:8080...");

    // Initialize Phase 1: eBPF Sensor
    if let Ok(mut ebpf_sensor) = infra::ebpf_sensor::EbpfSensor::new() {
        if let Err(e) = ebpf_sensor.start() {
            log::warn!("Failed to start eBPF Sensor: {:?}", e);
        } else {
            log::info!("eBPF Sensor started successfully.");
        }
    } else {
        log::warn!("Failed to initialize eBPF Sensor.");
    }

    api::run_api().await
}
