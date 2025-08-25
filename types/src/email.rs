use crate::type_error::TypeError;
use crate::string_utils::StringUtils;

use std::fmt::Display;
use std::str::FromStr;

pub struct Email(String);

impl Email {
    pub fn new(email: String) -> Self {
        let mut email = Email(email);
        email.sanitize();
        email
    }

    fn sanitize(&mut self) {
        self.0 = self.0.remove_spaces().to_lowercase();
    }

    pub fn validate(&mut self) -> Result<(), Vec<TypeError>> {
        self.sanitize();
        let mut errors: Vec<TypeError> = Vec::new();

        if self.0.has_whitespace() {
            errors.push(TypeError::InvalidFormat {
                field: "Email",
                reason: "email contem espaços em branco",
            });
        }

        let email_length = self.0.len();
        let invalid_size = email_length <= 5 || email_length > 320;
        if invalid_size {
            errors.push(TypeError::InvalidLength {
                field: "Email",
                got: email_length,
                max: 320,
                min: 6,
            })
        }

        let missing_at = self.0.find('@').is_none();
        if missing_at {
            errors.push(TypeError::MissingPart {
                field: "Email",
                part: "@",
            })
        }

        let missing_dot = self.0.rfind('.').is_none();
        if missing_dot {
            errors.push(TypeError::MissingPart {
                field: "Email",
                part: ". após o @",
            })
        }

        let is_email_invalid = errors.len() > 0;
        if is_email_invalid {
            return Err(errors);
        }

        Ok(())
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for Email {
    type Err = TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string()))
    }
}

impl From<&str> for Email {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for Email {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl PartialEq<Email> for Email {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Email {}
