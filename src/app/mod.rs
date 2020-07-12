use actix_web::{web};
  
use crate::{
    math::controller as mathController
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
        .service(
            web::scope("math")
                .service(mathController::prime)
        )
    );
}