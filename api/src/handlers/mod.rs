use actix_web::{get, HttpResponse, Responder};

const HEALTH_CHECK_RESPONSE: &str = "Hello, Actix";

#[get("/")]
pub async fn health_checker() -> impl Responder {
    HttpResponse::Ok().body(HEALTH_CHECK_RESPONSE)
}
