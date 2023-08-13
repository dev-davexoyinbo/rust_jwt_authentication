use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web::{self},
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::{ready, LocalBoxFuture, Ready};
use tokio::runtime::Runtime;

use crate::{
    auth::{models::AuthenticationInfo, utils::JwtUtils},
    models::dto_models::ResponseDTO,
    states::db_state::DBState,
};
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

                let db_state = req
                    .app_data::<web::Data<DBState>>()
                    .expect("Unable to find db pool");
                
                let rt = Runtime::new().unwrap();
                rt.block_on(async {
                    let record =
                        sqlx::query!("SELECT id, email, name FROM users WHERE email = $1", email)
                            .fetch_one(&db_state.pool)
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

pub struct RequireAuthMiddlewareInitializer;

impl<S, B> Transform<S, ServiceRequest> for RequireAuthMiddlewareInitializer
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = RequireAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        return ready(Ok(RequireAuthMiddleware { service }));
    }
}

pub struct RequireAuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequireAuthMiddleware<S>
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
        let http_request = req.request().clone();
        let extensions = http_request.extensions();
        let auth_info = extensions.get::<AuthenticationInfo>();

        match auth_info {
            None => {
                let response = req.into_response(
                    HttpResponse::Unauthorized()
                        .json(ResponseDTO::new("Unauthenticated").message("Unauthenticated"))
                        .map_into_boxed_body(),
                );

                Box::pin(async move { Ok(response.map_into_right_body()) })
            }
            Some(_) => {
                let fut: <S as Service<ServiceRequest>>::Future = self.service.call(req);

                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res.map_into_left_body())
                })
            }
        }
    }
}
