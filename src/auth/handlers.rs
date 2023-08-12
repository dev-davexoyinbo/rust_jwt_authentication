use super::models::JsonTokenClaims;
use crate::auth::repositories::Repository;
use crate::models::dto_models::ResponseDTO;
use crate::{auth::utils::JwtUtils, models::user_models::User};
use actix_web::{post, web, HttpResponse, Responder};
use chrono::{Duration, Utc};
use log::info;
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Postgres};

pub fn auth_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("api/auth").service(login));
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

#[post("login")]
pub async fn login(data: web::Json<LoginDTO>, db_pool: web::Data<PgPool>) -> impl Responder {
    let now = Utc::now();
    let LoginDTO { email, password } = data.into_inner();

    let user = sqlx::query("SELECT * from users WHERE email = $1")
        .bind(&email)
        .map(|row: PgRow| Repository::<User>::from_row(&row).unwrap())
        .fetch_one(&(**db_pool))
        .await;


    match user {
        Err(_) => HttpResponse::Unauthorized()
            .json(ResponseDTO::new("User does not exist").message("User does not exist")),
        Ok(user) => {
            if user.password != password {
                return HttpResponse::Unauthorized().json(ResponseDTO::new("Unauthorized").message("Invalid email or password"))
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
