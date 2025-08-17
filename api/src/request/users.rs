use core_types::{Cpf, Email, Name, Password, PhoneNumber, Status, Username};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: Name,
    pub cpf: Cpf,
    pub email: Email,
    pub password: Password,
    pub phone_number: Option<PhoneNumber>,
    pub status: Status,
}
 
#[derive(Deserialize)]
pub struct GetUserByIdRequest {
    pub id: u32,
}

#[derive(Deserialize)]
pub struct GetUserByEmailRequest {
    pub email: Email,
}

