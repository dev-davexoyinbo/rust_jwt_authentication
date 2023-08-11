use actix_web::{post, web, HttpResponse, Responder};
use chrono::{Duration, Utc};
use crate::models::dto_models::ResponseDTO;
use super::models::JsonTokenClaims;
use crate::auth::utils::JwtUtils;

pub fn auth_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("api/auth").service(login));
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct LoginDTO {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct LoginResponseDTO {
    pub token: String,
}

#[post("login")]
pub async fn login(data: web::Json<LoginDTO>) -> impl Responder {
    let now = Utc::now();

    let claims = JsonTokenClaims {
        sub: String::from("email@email.com"),
        iat: now.timestamp() as u64,
        exp: (now + Duration::minutes(60)).timestamp() as u64,
    };

    let response = match JwtUtils::encode(&claims) {
        Ok(token) => HttpResponse::Created().json(ResponseDTO::new(LoginResponseDTO { token })),
        Err(_) => HttpResponse::InternalServerError()
            .json(ResponseDTO::new("Unable to encode token").message("Unable to encode token")),
    };

    return response;
} //end function login
