use user::{CreateUserDTO, UserService};

fn main() {
    let user_service = UserService::new();
    let user_dto = match CreateUserDTO::new(
        "Mateus viana Maschietto".into(),
        "mateus@gmail.com".into(),
        "Password123".into(),
        "5531985470266".into(),
        "A".into()
    ) {
        Ok(dto) => dto,
        Err(errors) => {
            eprintln!("Falha na aplicação:");
            for error in errors {
                eprintln!("  - {}", error);
            }
            return;
        }
    };

    let user = user_service.create_user(user_dto);
    println!("{}:{}:{}", user.name, user.email, user.status);
}
