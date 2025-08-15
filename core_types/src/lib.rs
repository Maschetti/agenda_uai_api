pub mod email;
pub mod cpf;
pub mod password;
pub mod phone_number;
pub mod status;
pub mod username;
pub mod name;
pub mod error_message;

// Re-exports so other crates can `use core_types::Email;`
pub use email::Email;
pub use cpf::Cpf;
pub use password::Password;
pub use phone_number::PhoneNumber;
pub use status::Status;
pub use username::Username;
pub use name::Name;
pub use error_message::ErrorMessage;