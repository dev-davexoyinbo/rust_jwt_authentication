#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct JsonTokenClaims {
    pub sub: String,
    pub iat: u64,
    pub exp: u64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct AuthenticationInfo {
    pub id: u32,
    pub email: String,
    pub name: String,
}