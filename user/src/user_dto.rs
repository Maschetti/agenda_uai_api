use types::{Email, Name, PhoneNumber, Status, TypeError};

pub struct CreateUserDTO {
    pub name: Name,
    pub password: String,
    pub phone_number: PhoneNumber,
    pub email: Email,
    pub status: Status,
}

impl CreateUserDTO {
    pub fn new(
        mut name: Name,
        mut email: Email,
        password: String,
        mut phone_number: PhoneNumber,
        mut status: Status) -> Result<Self, Vec<TypeError>> {
        let mut all_errors: Vec<TypeError> = Vec::new();

        match email.validate() {
            Ok(_) => {},
            Err(mut errors) => {
                all_errors.append(&mut errors);
            },
        }

        match name.validate() {
            Ok(_) => {},
            Err(mut errors) => {
                all_errors.append(&mut errors);
            },
        }

        match status.validate() {
            Ok(_) => {}
            Err(mut errors) => {
                all_errors.append(&mut errors);
            }
        }

        match phone_number.validate() {
            Ok(_) => {}
            Err(mut errors) => {
                all_errors.append(&mut errors);
            }
        }

        if all_errors.len() > 0 {
            Err(all_errors)
        } else {
            Ok(Self {
                name,
                email,
                phone_number,
                password,
                status,
            })
        }
    }
}

pub struct GetUserByIdDTO {
    pub id: u32,
}

impl GetUserByIdDTO {
    pub fn new(id: u32) -> Result<Self, Vec<TypeError>> {
        let mut all_errors: Vec<TypeError> = Vec::new();

        if id == 0 {
            all_errors.push(TypeError::InvalidFormat { field: ("Id"), reason: ("deve ser maior que 0.") });
        }

        if all_errors.len() > 0 {
            Err(all_errors)
        } else {
            Ok(Self {
                id
            })
        }
    }
}
