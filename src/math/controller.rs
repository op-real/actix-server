use actix_web::{web, HttpResponse, Responder, get};

use super::service;

#[get("/prime/{number}")]
pub async fn prime(number: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().body(service::prime(number.into_inner()).to_string())
}
