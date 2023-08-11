use jsonwebtoken::{TokenData, decode, DecodingKey, Validation, encode, Header, EncodingKey};

use crate::configurations::app_configuration::AppConfiguration;

use super::models::JsonTokenClaims;

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

    pub fn encode(claims: &JsonTokenClaims) -> Result<String, jsonwebtoken::errors::Error> {
        let jwt_config = AppConfiguration::get_configuration().unwrap().jwt_config;

        return encode(
            &Header::default(),
            claims,
            &EncodingKey::from_secret(jwt_config.secret.as_ref()),
        );
    }
}
