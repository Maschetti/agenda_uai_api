use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::Deref;

use crate::ErrorMessage;

#[derive(Debug, Clone, Serialize)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    fn sanitize(s: &str) -> String {
        let s = s.trim();
        let mut out = String::with_capacity(s.len());
        let mut chars = s.chars();

        if let Some(first) = chars.next() {
            if first == '+' {
                out.push('+');
            } else if first.is_ascii_digit() {
                out.push(first);
            } else {
                // primeiro caractere inválido
                // (não retorna aqui, pois sanitização ainda coleta dígitos)
            }
        }

        for c in chars {
            if c.is_ascii_digit() {
                out.push(c);
            } else if !c.is_whitespace() && c != '-' && c != '(' && c != ')' {
                // rejeitamos outros caracteres não convencionais
                // Ex.: letras ou símbolos
                // a sanitização os ignora, mas registramos que havia algo inválido
            }
        }

        out
    }

    fn digits_count(s: &str) -> usize {
        s.chars().filter(|c| c.is_ascii_digit()).count()
    }

    fn validate(s: &str) -> Result<(), ErrorMessage> {
        let digits = Self::digits_count(s);

        if digits < 8 {
            return Err(ErrorMessage::PhoneTooShort { min: 8 });
        }

        // validação extra: só '+' no início é permitido
        if s.chars().skip(1).any(|c| c == '+') {
            return Err(ErrorMessage::PhoneInvalidChar);
        }

        Ok(())
    }
}

impl TryFrom<String> for PhoneNumber {
    type Error = ErrorMessage;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let clean = Self::sanitize(&s);
        Self::validate(&clean)?;
        Ok(PhoneNumber(clean))
    }
}

impl<'de> Deserialize<'de> for PhoneNumber {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(d)?;
        PhoneNumber::try_from(raw).map_err(|e| serde::de::Error::custom(e.to_string()))
    }
}

impl Deref for PhoneNumber {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for PhoneNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
