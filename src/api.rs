use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Status {
    system: &'static str,
    active: bool,
}

#[derive(Deserialize)]
struct ThreatInput {
    signature: String,
}

#[get("/api/status")]
async fn status() -> impl Responder {
    web::Json(Status { system: "online", active: true })
}

#[post("/api/threats")]
async fn receive_threat(info: web::Json<ThreatInput>) -> impl Responder {
    log::info!("Threat received: {}", info.signature);
    HttpResponse::Ok().body("Threat logged")
}

pub fn get_service() -> App<impl actix_web::dev::ServiceFactory<actix_web::dev::ServiceRequest, Config = (), Response = actix_web::dev::ServiceResponse, Error = actix_web::Error, InitError = ()>> {
    App::new()
        .app_data(web::JsonConfig::default().limit(4096).error_handler(|err, _req| {
            actix_web::error::InternalError::from_response(
                err,
                HttpResponse::BadRequest().body("Invalid input")
            ).into()
        }))
        .service(status)
        .service(receive_threat)
}

pub async fn run_api() -> std::io::Result<()> {
    HttpServer::new(|| get_service())
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
