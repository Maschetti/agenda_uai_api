use crate::CreateUserDTO;
use crate::GetUserByIdDTO;
use crate::User;

pub struct UserService {
    users: Vec<User>
}

impl UserService {
    pub fn new() -> UserService {
        Self {
            users: Vec::new()
        }
    }

    pub fn create_user(&mut self, dto: CreateUserDTO) {
        let user = User::new(dto);
        self.users.push(user);
    }

    pub fn get_user_by_id(&self, dto: GetUserByIdDTO) -> Option<&User> {
        self.users.iter().find(|user| user.id == dto.id)
    }
}
