use std::collections::HashMap;

use actix_web::{Responder, HttpResponse};

pub async fn healthcheck() -> impl Responder {
    return HttpResponse::Ok().json(HashMap::from([
        ("status", "running"),
        ("message", "all seems to be well")
    ]));
}//end function healthcheck