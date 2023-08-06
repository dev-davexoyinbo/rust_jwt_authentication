pub struct RegisterUserDTO {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub struct LoginUserDTO {
    pub email: String,
    pub password: String,
}