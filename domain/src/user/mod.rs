use core_types::{Cpf, Email, Name, Password, PhoneNumber, Status, Username};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: u32,
    pub username: Username,
    pub name: Name,
    pub cpf: Cpf,
    pub email: Email,
    pub token: String,
    #[serde(skip_serializing)]
    #[allow(dead_code)]
    pub password: Password,
    pub phone_number: PhoneNumber,
    pub status: Option<Status>,
}
