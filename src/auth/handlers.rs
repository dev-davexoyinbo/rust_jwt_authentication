use std::collections::HashMap;

use super::extractors::Authenticated;
use super::middlewares::require_auth_middleware::RequireAuthMiddlewareInitializer;
use super::models::JsonTokenClaims;
use crate::auth::dto::*;
use crate::auth::repositories::Repository;
use crate::models::dto_models::ResponseDTO;
use crate::models::user_models::FilteredUser;
use crate::states::db_state::DBState;
use crate::{auth::utils::JwtUtils, models::user_models::User};
use actix_web::{get, post, web, HttpResponse, Responder};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{Duration, Utc};

pub fn auth_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("api/auth")
            .service(login)
            .service(register)
            .service(
                web::scope("")
                    .wrap(RequireAuthMiddlewareInitializer)
                    .service(user_route),
            ),
    );
}

#[post("login")]
pub async fn login(data: web::Json<LoginDTO>, db_state: web::Data<DBState>) -> impl Responder {
    let now = Utc::now();
    let LoginDTO { email, password } = data.into_inner();

    let user = Repository::<User>::get_by_email(&db_state.pool, &email).await;

    match user {
        None => HttpResponse::Unauthorized()
            .json(ResponseDTO::new("User does not exist").message("User does not exist")),
        Some(user) => {
            let parsed_hash = PasswordHash::new(&user.password);

            if let Err(_) = parsed_hash {
                return HttpResponse::Unauthorized()
                    .json(ResponseDTO::new("Unauthorized").message("Unable to parse password"));
            }

            let password_correct = Argon2::default()
                .verify_password(password.as_bytes(), &(parsed_hash.unwrap()))
                .is_ok();

            if !password_correct {
                return HttpResponse::Unauthorized()
                    .json(ResponseDTO::new("Unauthorized").message("Invalid email or password"));
            }

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
pub async fn register(
    data: web::Json<RegisterDTO>,
    db_state: web::Data<DBState>,
) -> impl Responder {
    let RegisterDTO {
        email,
        password,
        name,
    } = data.into_inner();

    let salt = SaltString::generate(&mut OsRng);
    let password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    if Repository::<User>::exist_by_email(&db_state.pool, &email).await {
        return HttpResponse::NotFound()
            .json(ResponseDTO::new("Not Found").message("The email already exist"));
    }

    let user = Repository::<User>::create_one(
        &db_state.pool,
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
} //end function register

#[get("user")]
pub async fn user_route(auth: Authenticated, db_state: web::Data<DBState>) -> impl Responder {
    let user: Option<User> = Repository::<User>::get_by_email(&db_state.pool, &auth.email).await;

    match user {
        Some(user) => {
            let filtered_user: Result<FilteredUser, _> = user.try_into();
            match filtered_user {
                Ok(filtered_user) => HttpResponse::Ok()
                    .json(ResponseDTO::new(filtered_user).message("Authenticated user")),
                Err(e) => HttpResponse::InternalServerError()
                    .json(ResponseDTO::new(e.to_string()).message("Unable to build user response")),
            }
        }
        None => HttpResponse::NotFound()
            .json(ResponseDTO::new("Not found").message("User record not found")),
    }
} // end user function
