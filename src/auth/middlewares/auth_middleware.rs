use std::rc::Rc;

use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http,
    web::{self},
    Error, HttpMessage,
};
use futures_util::future::{ready, LocalBoxFuture, Ready};

use crate::{
    auth::{models::AuthenticationInfo, utils::JwtUtils},
    states::db_state::DBState,
};
pub struct AuthMiddlewareInitializer;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S: 'static, B> Transform<S, ServiceRequest> for AuthMiddlewareInitializer
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
        return ready(Ok(AuthMiddleware {
            service: Rc::new(service),
        }));
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<S>,
}

impl<S> AuthMiddleware<S> {
    pub async fn validate_auth_info() {}
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        Box::pin(async move {
            let http_request = req.request();
            // Get token from the authorization header
            let token = http_request
                .headers()
                .get(http::header::AUTHORIZATION)
                .map(|h| {
                    let authorization_header = h.to_str().expect("Unable to convert to str");
                    if authorization_header.len() > 7 {
                        let (_, token) = authorization_header.split_at(7);
                        token.to_string()
                    } else {
                        "".to_string()
                    }
                });

            if let Some(token) = token {
                // Decode token
                if let Ok(token_data) = JwtUtils::decode(&token) {
                    let claims = token_data.claims;
                    let email = claims.sub;

                    let db_state = req
                        .app_data::<web::Data<DBState>>()
                        .expect("Unable to find db pool");

                    let record =
                        sqlx::query!("SELECT id, email, name FROM users WHERE email = $1", email)
                            .fetch_one(&db_state.pool)
                            .await;

                    if let Ok(record) = record {
                        let auth_info = AuthenticationInfo {
                            id: record.id as u32,
                            email: record.email,
                            name: record.name,
                        };
                        req.request()
                            .extensions_mut()
                            .insert::<AuthenticationInfo>(auth_info);
                    };
                };
            };

            let res = service.call(req).await?;
            Ok(res.map_into_left_body())
        })
    }
}
