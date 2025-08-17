use core_types::{Cpf, Email, Name, Password, PhoneNumber, Status, Username};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: Username,
    pub name: Name,
    pub cpf: Cpf,
    pub email: Email,
    pub password: Password,
    pub phone_number: PhoneNumber,
    pub status: Option<Status>,
}

#[derive(Deserialize)]
pub struct GetUserRequest {
    pub username: Username,
    pub name: Name,
    pub cpf: Cpf,
    pub email: Email,
    pub password: Password,
    pub phone_number: PhoneNumber,
    pub status: Option<Status>,
}
