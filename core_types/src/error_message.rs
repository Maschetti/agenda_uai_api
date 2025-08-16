use std::fmt;

/// Enum geral para mensagens de erro de validação
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorMessage {
    // Gerais
    EmptyField { field: &'static str },
    TooShort { field: &'static str, min: usize },
    TooLong { field: &'static str, max: usize },
    InvalidChar { field: &'static str },

    // Email
    EmailNonAscii,
    EmailInvalidFormat,
    EmailMissingAt,
    EmailMissingDot,
    EmailDotBeforeAt,
    EmailEmptyLocal,
    EmailEmptyDomain,
    EmailTldTooShort { min: usize },

    // Password
    PasswordMissingLowercase,
    PasswordMissingUppercase,
    PasswordMissingDigit,
    PasswordMissingSymbol,

    // Name
    NameInvalidChar,

    CpfInvalidLength,  // len != 11 após sanitização
    CpfRepeatedDigits, // 00000000000, 11111111111, etc.
    CpfInvalidChecksum,

    PhoneTooShort { min: usize },
    PhoneInvalidChar,

    InvalidStatus,
}

impl fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ErrorMessage::*;
        match *self {
            EmptyField { field } => write!(f, "o campo '{field}' não pode ser vazio"),
            TooShort { field, min } => write!(
                f,
                "o campo '{field}' é muito curto (mínimo {min} caracteres)"
            ),
            TooLong { field, max } => write!(
                f,
                "o campo '{field}' é muito longo (máximo {max} caracteres)"
            ),
            InvalidChar { field } => write!(f, "o campo '{field}' contém caracteres inválidos"),

            EmailNonAscii => write!(f, "o e-mail deve conter apenas caracteres ASCII"),
            EmailInvalidFormat => write!(f, "o e-mail não está em um formato válido"),
            EmailMissingAt => write!(f, "o e-mail deve conter o caractere '@'"),
            EmailMissingDot => write!(
                f,
                "o domínio do e-mail deve conter um ponto (ex.: exemplo.com)"
            ),
            EmailDotBeforeAt => write!(f, "o ponto do domínio deve vir após o '@'"),
            EmailEmptyLocal => write!(f, "a parte local (antes do '@') não pode ser vazia"),
            EmailEmptyDomain => write!(f, "o domínio (após o '@') não pode ser vazio"),
            EmailTldTooShort { min } => write!(
                f,
                "o TLD (após o último ponto) deve ter pelo menos {min} caracteres"
            ),

            PasswordMissingLowercase => {
                write!(f, "a senha deve conter pelo menos uma letra minúscula")
            }
            PasswordMissingUppercase => {
                write!(f, "a senha deve conter pelo menos uma letra maiúscula")
            }
            PasswordMissingDigit => write!(f, "a senha deve conter pelo menos um dígito"),
            PasswordMissingSymbol => write!(f, "a senha deve conter pelo menos um símbolo"),

            NameInvalidChar => write!(f, "o nome contém caracteres inválidos"),

            CpfInvalidLength => write!(f, "CPF deve conter 11 dígitos"),
            CpfRepeatedDigits => write!(f, "CPF inválido: não pode conter todos os dígitos iguais"),
            CpfInvalidChecksum => write!(f, "CPF inválido: dígitos verificadores não conferem"),

            PhoneTooShort { min } => write!(f, "o telefone deve conter pelo menos {min} dígitos"),
            PhoneInvalidChar => write!(f, "o telefone contém caracteres inválidos"),

            InvalidStatus => write!(f, "status inválido (use 'A', 'I' ou 'R')"),
        }
    }
}

impl std::error::Error for ErrorMessage {}
