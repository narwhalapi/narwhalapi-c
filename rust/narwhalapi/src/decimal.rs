use crate::Error;
use serde::{Deserialize, Deserializer, Serialize};
use std::{fmt, str::FromStr};

/// An exact decimal wire string. Construction never rounds or changes its scale.
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
#[non_exhaustive]
pub struct Decimal(String);
impl Decimal {
    pub fn new(value: impl Into<String>) -> Result<Self, Error> {
        let value = value.into();
        let digits = value.strip_prefix('-').unwrap_or(&value);
        let mut parts = digits.split('.');
        let valid = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
        if !valid(parts.next().unwrap_or_default())
            || parts.next().is_some_and(|p| !valid(p))
            || parts.next().is_some()
        {
            return Err(Error::InvalidArgument("expected a decimal wire string"));
        }
        Ok(Self(value))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
    /// Exact conversion; more than 28 significant digits or excess scale is an error.
    #[cfg(feature = "rust_decimal")]
    #[cfg_attr(docsrs, doc(cfg(feature = "rust_decimal")))]
    pub fn to_rust_decimal(&self) -> Result<rust_decimal::Decimal, rust_decimal::Error> {
        let digits: String = self
            .0
            .bytes()
            .filter(u8::is_ascii_digit)
            .map(char::from)
            .collect();
        if digits.trim_start_matches('0').len() > 28 {
            return Err(rust_decimal::Error::ExceedsMaximumPossibleValue);
        }
        rust_decimal::Decimal::from_str_exact(&self.0)
    }
}
impl FromStr for Decimal {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        Self::new(s)
    }
}
impl TryFrom<&str> for Decimal {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self, Error> {
        Self::new(s)
    }
}
impl AsRef<str> for Decimal {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl fmt::Debug for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Decimal").field(&self.0).finish()
    }
}
impl<'de> Deserialize<'de> for Decimal {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Self::new(String::deserialize(d)?).map_err(serde::de::Error::custom)
    }
}
