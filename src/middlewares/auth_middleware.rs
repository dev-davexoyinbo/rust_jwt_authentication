use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, ResponseError,
};
use futures_util::future::{ready, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use sqlx::PgPool;
use tokio::runtime::Runtime;

use crate::{auth::models::{AuthenticationInfo, JsonTokenClaims, JwtUtils}, configurations::app_configuration::AppConfiguration};
pub struct AuthMiddlewareInitializer;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for AuthMiddlewareInitializer
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        return ready(Ok(AuthMiddleware { service }));
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let http_request = req.request();

        // Get token from the authorization header
        let token = http_request
            .headers()
            .get("Authorization")
            .map(|h| h.to_str().unwrap().split_at(7).1.to_string());

        if let Some(token) = token {
            // Decode token
            if let Ok(token_data) = JwtUtils::decode(&token) {
                let claims = token_data.claims;
                let email = claims.sub;

                let db_pool = req.app_data::<PgPool>().expect("Unable to find db pool");

                let rt = Runtime::new().unwrap();
                rt.block_on(async {
                    let record =
                        sqlx::query!("SELECT id, email, name FROM users WHERE email = $1", email)
                            .fetch_one(db_pool)
                            .await;

                    if let Ok(record) = record {
                        http_request.extensions_mut().insert::<AuthenticationInfo>(
                            AuthenticationInfo {
                                id: record.id as u32,
                                email: record.email,
                                name: record.name,
                            },
                        );
                    };
                });
            };
        };

        let fut: <S as Service<ServiceRequest>>::Future = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res.map_into_left_body())
        })
    }
}

#[derive(Debug, derive_more::Display, derive_more::Error, serde::Deserialize, serde::Serialize)]
// #[display(fmt = "Authentication error: {}", message)]
struct AuthenticationError {
    message: String,
}

// Use default implementation for `error_response()` method
impl ResponseError for AuthenticationError {}

// let is_authenticated = false;
// if !is_authenticated {
//     let response = req.into_response(
//         HttpResponse::Unauthorized()
//             .json(ResponseDTO::new("Unauthenticated").message("Unauthenticated"))
//             .map_into_boxed_body(),
//     );

//     Box::pin(async { Ok(response.map_into_right_body()) })
// } else {
//     let fut: <S as Service<ServiceRequest>>::Future = self.service.call(req);

//     Box::pin(async move {
//         let res = fut.await?;
//         Ok(res.map_into_left_body())
//     })
// }
