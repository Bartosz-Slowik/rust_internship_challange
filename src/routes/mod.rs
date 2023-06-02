mod search;

use actix_web::{get, HttpResponse, Responder};

#[get("/healthcheck")]
pub async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("I am alive!")
}
