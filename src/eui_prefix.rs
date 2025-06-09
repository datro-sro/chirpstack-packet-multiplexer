use std::fmt;
use std::str::FromStr;

use serde::{de::{self, Visitor}, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct EuiPrefix([u8; 8], u32);

impl EuiPrefix {
    pub fn new(prefix: [u8; 8], size: u32) -> Self {
        EuiPrefix(prefix, size)
    }

    pub fn is_match(&self, eui_le: [u8; 8]) -> bool {
        let eui = u64::from_le_bytes(eui_le);
        let prefix = u64::from_be_bytes(self.0);
        eui >> (64 - self.1) == prefix >> (64 - self.1)
    }
}

impl fmt::Display for EuiPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", hex::encode(self.0), self.1)
    }
}

impl fmt::Debug for EuiPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", hex::encode(self.0), self.1)
    }
}

#[derive(Debug)]
pub enum ParseEuiPrefixError {
    Format,
    Size,
    Hex(hex::FromHexError),
}

impl fmt::Display for ParseEuiPrefixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseEuiPrefixError::Format => write!(f, "EuiPrefix must be in the form 0000000000000000/0"),
            ParseEuiPrefixError::Size => write!(f, "EuiPrefix max prefix size is 64"),
            ParseEuiPrefixError::Hex(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for ParseEuiPrefixError {}

impl From<hex::FromHexError> for ParseEuiPrefixError {
    fn from(e: hex::FromHexError) -> Self {
        ParseEuiPrefixError::Hex(e)
    }
}

impl FromStr for EuiPrefix {
    type Err = ParseEuiPrefixError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() != 2 || parts[0].len() != 16 {
            return Err(ParseEuiPrefixError::Format);
        }

        let mut mask = [0u8; 8];
        hex::decode_to_slice(parts[0], &mut mask)?;
        let size: u32 = parts[1].parse().map_err(|_| ParseEuiPrefixError::Format)?;
        if size > 64 {
            return Err(ParseEuiPrefixError::Size);
        }
        Ok(EuiPrefix(mask, size))
    }
}

impl Serialize for EuiPrefix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for EuiPrefix {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EuiPrefixVisitor;

        impl<'de> Visitor<'de> for EuiPrefixVisitor {
            type Value = EuiPrefix;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("An EuiPrefix in the format 0000000000000000/0 is expected")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                EuiPrefix::from_str(value).map_err(|e| E::custom(format!("{}", e)))
            }
        }

        deserializer.deserialize_str(EuiPrefixVisitor)
    }
}
