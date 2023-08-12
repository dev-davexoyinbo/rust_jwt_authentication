
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct LoginDTO {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct LoginResponseDTO {
    pub token: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct RegisterDTO {
    pub email: String,
    pub password: String,
    pub name: String,
}