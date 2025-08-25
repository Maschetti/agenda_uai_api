use crate::{TypeError};
use std::fmt::Display;
use std::str::FromStr;
use crate::string_utils::StringUtils;

pub struct Name(String);

impl Name {
    pub fn new(name: String) -> Self {
        let mut name = Name(name);
        name.sanitize();
        name
    }

    pub fn sanitize(&mut self) {
        self.0 = self.0.trim().to_lowercase().normalize_spaces();
        self.proper_name();
    }

    pub fn proper_name(&mut self) -> &mut Self {
        let mut result = String::new();
        let mut capitalize_next = true;

        for c in self.0.chars() {
            if capitalize_next {
                // pega o primeiro char de to_uppercase (pode ter mais de 1 char, ex: ß → SS)
                result.extend(c.to_uppercase());
                capitalize_next = false;
            } else {
                result.extend(c.to_lowercase());
            }

            if c.is_whitespace() {
                capitalize_next = true;
            }
        }

        self.0 = result;
        self
    }

    pub fn validate(&mut self) -> Result<(), Vec<TypeError>> {
        self.sanitize();

        let mut errors: Vec<TypeError> = Vec::new();

        let name_length = self.0.len();
        let invalid_length = name_length < 3 || name_length >= 250;
        if invalid_length {
            errors.push(TypeError::InvalidLength {
                field: "Name",
                max: 250,
                min: 3,
                got: name_length,
            })
        }

        if self.0.has_non_letters_or_spaces() {
            errors.push(TypeError::InvalidChars {
                field: "Name",
            })
        }

        if errors.len() > 0 {
            Err(errors)
        } else {
            Ok(())
        }

    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for Name {
    type Err = TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string()))
    }
}

impl From<&str> for Name {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for Name {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl PartialEq<Name> for Name {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Name {}
