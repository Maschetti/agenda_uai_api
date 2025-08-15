use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{Display, Formatter};

use crate::ErrorMessage;

#[derive(Debug, Clone, Serialize)]
pub enum Status { A, I, R }

impl TryFrom<String> for Status {
    type Error = ErrorMessage;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.trim().to_ascii_uppercase().as_str() {
            "A" => Ok(Status::A),
            "I" => Ok(Status::I),
            "R" => Ok(Status::R),
            _ => Err(ErrorMessage::InvalidStatus),
        }
    }
}

impl<'de> Deserialize<'de> for Status {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(d)?;
        Status::try_from(raw).map_err(|e| serde::de::Error::custom(e.to_string()))
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Status::A => "A",
            Status::I => "I",
            Status::R => "R",
        })
    }
}
