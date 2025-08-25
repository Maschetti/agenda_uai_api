use thiserror::Error;

#[derive(Debug, Error)]
pub enum TypeError {
    #[error("{field}: vazio.")]
    Empty { field: &'static str },

    #[error("{field}: formato inválido{reason}.")]
    InvalidFormat {
        field: &'static str,
        reason: &'static str,
    },

    #[error("{field}: muito curto (min {min}, got {got}).")]
    TooShort {
        field: &'static str,
        min: usize,
        got: usize,
    },

    #[error("{field}: muito longo (max {max}, got {got}.)")]
    TooLong {
        field: &'static str,
        max: usize,
        got: usize,
    },

    #[error("{field}: deve ter entre (max {max}, min {min}, got {got}).")]
    InvalidLength {
        field: &'static str,
        max: usize,
        min: usize,
        got: usize,
    },

    #[error("{field}: caracteres inválidos.")]
    InvalidChars { field: &'static str },

    #[error("{field}: está faltando o(a) {part}.")]
    MissingPart {
        field: &'static str,
        part: &'static str,
    },

    #[error("Error de de conversão.")]
    ConversionError,
}