use core_types::{Cpf, PhoneNumber, Name, Status};
use serde::Serialize;

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub token: String,
}

#[derive(Serialize)]
pub struct GetUserByEmailResponse {
    pub id: u32,
    pub name: Name,
    pub phone_number: Option<PhoneNumber>,
    pub cpf: Cpf,
    pub status: Status,
    pub token: String,
}

#[derive(Serialize)]
pub struct GetUserByIdResponse {
    pub id: u32,
    pub name: Name,
    pub phone_number: Option<PhoneNumber>,
    pub cpf: Cpf,
    pub status: Status,
    pub token: String,
}
