use serde::Serialize;
use core_types::{Cpf, Email, Password, PhoneNumber, Status};

#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub cpf: Cpf,
    pub email: Email,
    #[serde(skip_serializing)]
    #[allow(dead_code)] 
    pub password: Password,
    pub phone_number: PhoneNumber,
    pub status: Status,
}