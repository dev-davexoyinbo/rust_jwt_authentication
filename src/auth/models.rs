use jsonwebtoken::{decode, DecodingKey, Validation, TokenData};

use crate::configurations::app_configuration::AppConfiguration;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct JsonTokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct AuthenticationInfo {
    pub id: u32,
    pub email: String,
    pub name: String,
}

pub struct JwtUtils;
impl JwtUtils {
    pub fn decode(token: &str) -> Result<TokenData<JsonTokenClaims>, jsonwebtoken::errors::Error> {
        let jwt_config = AppConfiguration::get_configuration().unwrap().jwt_config;

        return decode::<JsonTokenClaims>(
            &token,
            &DecodingKey::from_secret(jwt_config.secret.as_ref()),
            &Validation::default(),
        );
    }
}
