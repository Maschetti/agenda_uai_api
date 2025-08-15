use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::Deref;

use crate::ErrorMessage;

#[derive(Debug, Clone, Serialize)]
pub struct Cpf(String);

impl Cpf {
    fn sanitize(s: &str) -> String {
        s.chars().filter(|c| c.is_ascii_digit()).collect()
    }

    fn validate_digits(d: &str) -> Result<(), ErrorMessage> {
        // 1) tamanho exato
        if d.len() != 11 {
            return Err(ErrorMessage::CpfInvalidLength);
        }

        // 2) não pode ser todos os dígitos iguais
        if d.as_bytes().windows(2).all(|w| w[0] == w[1]) {
            return Err(ErrorMessage::CpfRepeatedDigits);
        }

        // 3) checagem dos dígitos verificadores
        fn dv(slice: &[u8], start: u32) -> u8 {
            let sum: u32 = slice
                .iter()
                .enumerate()
                .map(|(i, &b)| ((b - b'0') as u32) * (start - i as u32))
                .sum();
            let r = (sum * 10) % 11;
            if r == 10 { 0 } else { r as u8 }
        }

        let b = d.as_bytes();
        let d1 = dv(&b[0..9], 10);
        let d2 = dv(&b[0..10], 11);
        let ok = d1 == b[9] - b'0' && d2 == b[10] - b'0';

        if !ok {
            return Err(ErrorMessage::CpfInvalidChecksum);
        }

        Ok(())
    }
}

impl TryFrom<String> for Cpf {
    type Error = ErrorMessage;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let digits = Self::sanitize(&s);
        Self::validate_digits(&digits)?;
        Ok(Cpf(digits))
    }
}

impl<'de> Deserialize<'de> for Cpf {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(d)?;
        Cpf::try_from(raw).map_err(|e| serde::de::Error::custom(e.to_string()))
    }
}

impl Deref for Cpf {
    type Target = str;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl Display for Cpf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
