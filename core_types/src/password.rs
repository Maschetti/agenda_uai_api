use std::{fmt, str::FromStr};
use serde::{Deserialize, Deserializer};

use crate::ErrorMessage;

#[derive(Clone)]
pub struct Password(String);

// Não exponha a senha em Debug
impl fmt::Debug for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Password").field(&"***").finish()
    }
}

impl Password {
    pub fn new(s: &str) -> Result<Self, ErrorMessage> {
        // Política ASCII explícita (se quiser aceitar Unicode, remova este bloco)
        if !s.is_ascii() {
            return Err(ErrorMessage::InvalidChar { field: "senha" });
        }

        // Como é ASCII, len() em bytes == chars
        if s.len() < 6 {
            return Err(ErrorMessage::TooShort { field: "senha", min: 6 });
        }

        let has_lower  = s.bytes().any(|c| (b'a'..=b'z').contains(&c));
        let has_upper  = s.bytes().any(|c| (b'A'..=b'Z').contains(&c));
        let has_digit  = s.bytes().any(|c| (b'0'..=b'9').contains(&c));
        let has_symbol = s.bytes().any(|c| !c.is_ascii_alphanumeric());

        if !has_lower  { return Err(ErrorMessage::PasswordMissingLowercase); }
        if !has_upper  { return Err(ErrorMessage::PasswordMissingUppercase); }
        if !has_digit  { return Err(ErrorMessage::PasswordMissingDigit); }
        if !has_symbol { return Err(ErrorMessage::PasswordMissingSymbol); }

        // sem trim — mantém exatamente o que o usuário enviou
        Ok(Self(s.to_owned()))
    }

    pub fn as_str(&self) -> &str { &self.0 }
}

// Ergonomia
impl FromStr for Password {
    type Err = ErrorMessage;
    fn from_str(s: &str) -> Result<Self, Self::Err> { Self::new(s) }
}
impl TryFrom<String> for Password {
    type Error = ErrorMessage;
    fn try_from(s: String) -> Result<Self, Self::Error> { Self::new(&s) }
}

// Mensagem final já “limpa” em PT-BR vem do Display de ErrorMessage
impl<'de> Deserialize<'de> for Password {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw: String = String::deserialize(d)?;
        Self::new(&raw).map_err(|e| serde::de::Error::custom(e.to_string()))
    }
}
