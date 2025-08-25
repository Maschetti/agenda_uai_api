use std::fmt::Display;
use std::str::FromStr;
use crate::{TypeError};
use crate::string_utils::StringUtils;

pub struct Status(String);

impl Status {
    pub fn new(status: String) -> Status {
        let mut status = Status(status);
        status.sanitize();
        status
    }

    pub fn sanitize(&mut self) {
        self.0 = self.0.normalize_spaces().to_uppercase();
    }

    pub fn validate(&mut self) -> Result<(), Vec<TypeError>> {
        self.sanitize();

        let mut errors: Vec<TypeError> = Vec::new();

        if self.0.has_non_letters_or_spaces() {
            errors.push(TypeError::InvalidChars {
                field: "Status"
            })
        }

        let status_length = self.0.len();
        if status_length <= 0 || status_length > 3 {
            errors.push(
                TypeError::InvalidLength {
                    field: "Status",
                    got: status_length,
                    max: 3,
                    min: 1,
                }
            );
        }

        if errors.len() > 0 {
            return Err(errors);
        }
        Ok(())
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for Status {
    type Err = TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string()))
    }
}

impl From<&str> for Status {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for Status {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl PartialEq<Status> for Status {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Status {}