use std::collections::HashMap;

use super::models::JsonTokenClaims;
use crate::auth::repositories::Repository;
use crate::models::dto_models::ResponseDTO;
use crate::{auth::utils::JwtUtils, models::user_models::User};
use actix_web::{post, web, HttpResponse, Responder};
use chrono::{Duration, Utc};
use log::info;
use sqlx::PgPool;

pub fn auth_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("api/auth").service(login).service(register));
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct LoginDTO {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct LoginResponseDTO {
    pub token: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct RegisterDTO {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[post("login")]
pub async fn login(data: web::Json<LoginDTO>, db_pool: web::Data<PgPool>) -> impl Responder {
    let now = Utc::now();
    let LoginDTO { email, password } = data.into_inner();

    let user = Repository::<User>::get_by_email(&(**db_pool), &email).await;

    match user {
        None => HttpResponse::Unauthorized()
            .json(ResponseDTO::new("User does not exist").message("User does not exist")),
        Some(user) => {
            if user.password != password {
                return HttpResponse::Unauthorized()
                    .json(ResponseDTO::new("Unauthorized").message("Invalid email or password"));
            }

            info!("User ({}) loggedin successfully", &user.email);

            let claims = JsonTokenClaims {
                sub: user.email,
                iat: now.timestamp() as u64,
                exp: (now + Duration::minutes(60)).timestamp() as u64,
            };

            let response = match JwtUtils::encode(&claims) {
                Ok(token) => {
                    HttpResponse::Created().json(ResponseDTO::new(LoginResponseDTO { token }))
                }
                Err(_) => HttpResponse::InternalServerError().json(
                    ResponseDTO::new("Unable to encode token").message("Unable to encode token"),
                ),
            };

            return response;
        }
    }
} //end function login

#[post("register")]
pub async fn register(data: web::Json<RegisterDTO>, db_pool: web::Data<PgPool>) -> impl Responder {
    let RegisterDTO {
        email,
        password,
        name,
    } = data.into_inner();

    if Repository::<User>::exist_by_email(&db_pool, &email).await {
        return HttpResponse::NotFound()
            .json(ResponseDTO::new("Not Found").message("The email already exist"));
    }

    let user = Repository::<User>::create_one(
        &(**db_pool),
        &email.as_str(),
        &password.as_str(),
        &name.as_str(),
    )
    .await;

    match user {
        Err(e) => HttpResponse::BadRequest().json(ResponseDTO::new(e.to_string())),
        Ok(None) => HttpResponse::BadRequest().json(ResponseDTO::new("Unable to create User")),
        Ok(Some(user)) => HttpResponse::Created().json(
            ResponseDTO::new(HashMap::from([("id", user.id)])).message("User created successfully"),
        ),
    }
}
