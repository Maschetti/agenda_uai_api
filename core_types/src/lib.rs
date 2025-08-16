pub mod cpf;
pub mod email;
pub mod error_message;
pub mod name;
pub mod password;
pub mod phone_number;
pub mod status;
pub mod username;

// Re-exports so other crates can `use core_types::Email;`
pub use cpf::Cpf;
pub use email::Email;
pub use error_message::ErrorMessage;
pub use name::Name;
pub use password::Password;
pub use phone_number::PhoneNumber;
pub use status::Status;
pub use username::Username;
