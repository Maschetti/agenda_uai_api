use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::Deref;

use crate::ErrorMessage;

#[derive(Debug, Clone, Serialize)]
pub struct Username(String);

impl Username {
    fn sanitize(s: &str) -> String {
        s.trim().to_string()
    }

    fn validate(s: &str) -> Result<(), ErrorMessage> {
        let len = s.chars().count();

        if len == 0 {
            return Err(ErrorMessage::EmptyField { field: "username" });
        }
        if len < 3 {
            return Err(ErrorMessage::TooShort { field: "username", min: 3 });
        }
        if len > 32 {
            return Err(ErrorMessage::TooLong { field: "username", max: 32 });
        }
        // Apenas ASCII alfanumérico e os símbolos "._-"
        let ok = s.chars().all(|c| c.is_ascii_alphanumeric() || "_.-".contains(c));
        if !ok {
            return Err(ErrorMessage::InvalidChar { field: "username" });
        }

        Ok(())
    }
}

impl TryFrom<String> for Username {
    type Error = ErrorMessage;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let clean = Self::sanitize(&s);
        Self::validate(&clean)?;
        Ok(Username(clean))
    }
}

impl<'de> Deserialize<'de> for Username {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw: String = String::deserialize(d)?;
        Username::try_from(raw).map_err(|e| serde::de::Error::custom(e.to_string()))
    }
}

impl Deref for Username {
    type Target = str;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl Display for Username {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
