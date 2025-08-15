use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::Deref;

use crate::ErrorMessage;

#[derive(Debug, Clone, Serialize)]
pub struct Name(String);

impl Name {
    fn normalize_spaces(s: &str) -> String {
        let mut out = String::with_capacity(s.len());
        let mut last_space = false;
        for ch in s.trim().chars() {
            if ch.is_whitespace() {
                if !last_space {
                    out.push(' ');
                    last_space = true;
                }
            } else {
                out.push(ch);
                last_space = false;
            }
        }
        out
    }

    /// Primeira letra e toda letra após espaço em maiúscula.
    fn titlecase_after_space(s: &str) -> String {
        let mut out = String::with_capacity(s.len());
        let mut at_word_start = true; // início da string ou logo após espaço
        for ch in s.chars() {
            if at_word_start && ch.is_alphabetic() {
                // to_uppercase() pode render múltiplos chars (Unicode); coletamos todos
                for up in ch.to_uppercase() {
                    out.push(up);
                }
                at_word_start = false;
            } else {
                out.push(ch);
                // mantém caixa dos demais caracteres como vieram
                at_word_start = false;
            }
            if ch == ' ' {
                at_word_start = true;
            }
        }
        out
    }

    fn sanitize(s: &str) -> String {
        let spaced = Self::normalize_spaces(s);
        Self::titlecase_after_space(&spaced)
    }

    fn validate(s: &str) -> Result<(), ErrorMessage> {
        let len = s.chars().count();
        if len == 0 {
            return Err(ErrorMessage::EmptyField { field: "nome" });
        }
        if len > 250 {
            return Err(ErrorMessage::TooLong { field: "nome", max: 250 });
        }
        let ok = s.chars().all(|c|
            c.is_alphabetic() || c == ' ' || c == '\'' || c == '-' || c == '.'
        );
        if !ok {
            return Err(ErrorMessage::InvalidChar { field: "nome" });
        }
        Ok(())
    }
}

impl TryFrom<String> for Name {
    type Error = ErrorMessage;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let clean = Self::sanitize(&s);
        Self::validate(&clean)?;
        Ok(Name(clean))
    }
}

impl<'de> Deserialize<'de> for Name {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(d)?;
        Name::try_from(raw).map_err(|e| serde::de::Error::custom(e.to_string()))
    }
}

impl Deref for Name {
    type Target = str;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
