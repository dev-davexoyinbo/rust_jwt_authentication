use actix_web::{
    body::{BoxBody, EitherBody},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse, ResponseError,
};
use futures_util::future::{ready, Lazy, LocalBoxFuture, Ready};
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
        log::error!("Hi from start. You requested: {}", req.path());

        let is_authenticated = true;

        if !is_authenticated {
            let response = req.into_response(
                HttpResponse::Unauthorized()
                    .body("This")
                    .map_into_boxed_body(),
            );
            
            Box::pin(async { Ok(response.map_into_right_body()) })
        } else {
            let fut: <S as Service<ServiceRequest>>::Future = self.service.call(req);

            Box::pin(async move {
                let res = fut.await?;

                // log::error!("Hi from response");
                Ok(res.map_into_left_body())
            })
        }
    }
}

#[derive(Debug, derive_more::Display, derive_more::Error, serde::Deserialize, serde::Serialize)]
// #[display(fmt = "Authentication error: {}", message)]
struct AuthenticationError {
    message: String,
}

// Use default implementation for `error_response()` method
impl ResponseError for AuthenticationError {}
