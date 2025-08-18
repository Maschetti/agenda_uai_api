use core_types::{Cpf, Email, Name, Password, PhoneNumber, Status, Username};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateUserRequest {
    pub name: Name,
    pub cpf: Cpf,
    pub email: Email,
    pub password: Password,
    pub phone_number: Option<PhoneNumber>,
    pub status: Status,
}
 
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetUserByIdRequest {
    pub id: u32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetUserByEmailRequest {
    pub email: Email,
}

