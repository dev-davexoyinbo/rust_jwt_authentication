use std::{ops::Deref, pin::Pin};

use actix_web::{FromRequest, HttpMessage, error::ErrorUnauthorized};
use futures_util::Future;

use crate::models::dto_models::ResponseDTO;

use super::models::AuthenticationInfo;

pub struct Authenticated {
    value: AuthenticationInfo,
}

impl Authenticated {
    pub fn new(auth_info: AuthenticationInfo) -> Self {
        Authenticated { value: auth_info }
    }
}

impl Deref for Authenticated {
    type Target = AuthenticationInfo;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FromRequest for Authenticated {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>> + Send>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let auth_info = req.extensions().get::<AuthenticationInfo>().cloned();

        return Box::pin(async {
            match auth_info {
                Some(auth_info) => Ok(Authenticated::new(auth_info)),
                None => {
                    Err(ErrorUnauthorized(ResponseDTO::new("Unauthorized").message("Authentication required for this resource")))
                }
            }
        });
    }
}
