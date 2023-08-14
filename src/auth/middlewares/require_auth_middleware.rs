use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::{ready, LocalBoxFuture, Ready};

use crate::{models::dto_models::ResponseDTO, auth::models::AuthenticationInfo};
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
        let auth_info = req
            .request()
            .extensions()
            .get::<AuthenticationInfo>()
            .cloned();

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
