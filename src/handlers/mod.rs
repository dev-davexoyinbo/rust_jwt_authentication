use actix_web::{Responder, HttpResponse};

use crate::models::dto_models::ResponseDTO;

pub mod healthcheck;

pub async fn authenticated_route() -> impl Responder {
    return HttpResponse::Ok().json(ResponseDTO::new("This is the authenticated route"));
}