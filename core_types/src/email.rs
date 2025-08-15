use std::{str::FromStr};
use serde::{Deserialize, Deserializer, Serialize};

use crate::ErrorMessage;

#[derive(Debug, Clone, Serialize)]
pub struct Email(String);

impl Email {
    fn sanitize(s: &str) -> String {
        s.trim().to_ascii_lowercase()
    }

    fn is_ascii_basic(s: &str) -> bool {
        s.is_ascii()
    }

    fn has_no_spaces_or_controls(s: &str) -> bool {
        s.chars().all(|c| !c.is_control() && !c.is_whitespace())
    }

    fn validate(clean: &str) -> Result<(), ErrorMessage> {
        const MIN_LEN: usize = 5;
        const MIN_TLD: usize = 2;

        if clean.len() < MIN_LEN {
            return Err(ErrorMessage::TooShort { field: "e-mail", min: MIN_LEN });
        }
        if !Self::is_ascii_basic(clean) {
            // se preferir uma variante específica (e.g. NonAscii { field }), troque aqui
            return Err(ErrorMessage::InvalidChar { field: "e-mail" });
        }
        if !Self::has_no_spaces_or_controls(clean) {
            return Err(ErrorMessage::InvalidChar { field: "e-mail" });
        }

        let at = clean.find('@').ok_or(ErrorMessage::EmailMissingAt)?;
        let dot = clean.rfind('.').ok_or(ErrorMessage::EmailMissingDot)?;
        if at > dot {
            return Err(ErrorMessage::EmailDotBeforeAt);
        }

        let local = &clean[..at];
        let domain = &clean[at + 1..];
        if local.is_empty() {
            return Err(ErrorMessage::EmailEmptyLocal);
        }
        if domain.is_empty() {
            return Err(ErrorMessage::EmailEmptyDomain);
        }

        let tld = &clean[dot + 1..];
        if tld.len() < MIN_TLD {
            return Err(ErrorMessage::EmailTldTooShort { min: MIN_TLD });
        }

        Ok(())
    }

    pub fn new(s: &str) -> Result<Self, ErrorMessage> {
        let clean = Self::sanitize(s);
        Self::validate(&clean)?;
        Ok(Email(clean))
    }

    pub fn as_str(&self) -> &str { &self.0 }
}

impl FromStr for Email {
    type Err = ErrorMessage;
    fn from_str(s: &str) -> Result<Self, Self::Err> { Email::new(s) }
}

impl TryFrom<String> for Email {
    type Error = ErrorMessage;
    fn try_from(s: String) -> Result<Self, Self::Error> { Email::new(&s) }
}

impl<'de> Deserialize<'de> for Email {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw: String = String::deserialize(d)?;
        Email::new(&raw).map_err(|e| serde::de::Error::custom(e.to_string()))
    }
}
