use serde::Serialize;

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub token: String,
}

pub struct GetUserByEmailResponse {
    pub name: Name,
    pub phone_number: Option<PhoneNumber>,
    pub cpf: Cpf,
    pub status: Status,
    pub token: String,
}

pub struct GetUserByIdResponse {
    pub name: Name,
    pub phone_number: Option<PhoneNumber>,
    pub cpf: Cpf,
    pub status: Status,
    pub token: String,
}