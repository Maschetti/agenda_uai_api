use core_types::{Cpf, Email, Name, Password, PhoneNumber, Status};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: u32,
    pub name: Name,
    pub cpf: Cpf,
    pub email: Email,
    pub token: String,
    #[serde(skip_serializing)]
    #[allow(dead_code)]
    pub password: Password,
    pub phone_number: Option<PhoneNumber>,
    pub status: Status,
}
