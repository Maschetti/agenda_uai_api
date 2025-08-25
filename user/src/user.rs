use crate::CreateUserDTO;
use types::{Email, Name, PhoneNumber, Status};

pub struct User {
    pub id: u32,
    pub name: Name,
    pub password: String,
    pub phone_number: PhoneNumber,
    pub email: Email,
    pub status: Status
}

impl User {
    pub(crate) fn new(dto: CreateUserDTO) -> User {
        User {
            id: 0,
            name: dto.name,
            password: dto.password,
            email: dto.email,
            phone_number: dto.phone_number,
            status: dto.status
        }
    }
}
