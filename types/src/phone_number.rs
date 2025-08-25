use crate::{TypeError};
use std::fmt::Display;
use std::str::FromStr;
use crate::string_utils::StringUtils;

pub struct PhoneNumber(String);

impl PhoneNumber {
    pub fn new(s: String) -> Self {
        let mut phone_number = PhoneNumber(s);
        phone_number.sanitize();
        phone_number
    }

    pub fn sanitize(&mut self) {
        self.0 = self.0.to_lowercase().normalize_spaces().only_digits();
    }

    pub fn validate(&mut self) -> Result<(), Vec<TypeError>> {
        self.sanitize();
        let mut errors: Vec<TypeError> = Vec::new();

        let phone_number_length = self.0.len();
        let invalid_length = phone_number_length < 7 || phone_number_length >= 15;
        if invalid_length {
            errors.push(TypeError::InvalidLength {
                field: "Phone number",
                max: 15,
                min: 7,
                got: phone_number_length,
            })
        }

        if errors.len() > 0 {
            Err(errors)
        } else {
            Ok(())
        }
    }
}

impl Display for PhoneNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for PhoneNumber {
    type Err = TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string()))
    }
}

impl From<&str> for PhoneNumber {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for PhoneNumber {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl PartialEq<PhoneNumber> for PhoneNumber {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for PhoneNumber {}