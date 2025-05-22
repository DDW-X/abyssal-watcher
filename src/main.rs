mod api;
mod logs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logs::init_syslog();
    println!("Starting Abyssal Watcher backend on 0.0.0.0:8080...");
    api::run_api().await
}
