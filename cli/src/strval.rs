use std::{error::Error, fmt::Display};

use stellar_contract_env_host::{Host, IntoVal, RawVal, TryFromVal};

#[derive(Debug)]
pub enum StrValError {
    UnknownError,
    NoType,
    UnknownType,
    InvalidNumberOfParts,
    InvalidValue,
}

impl Error for StrValError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::UnknownError => None,
            Self::NoType => None,
            Self::UnknownType => None,
            Self::InvalidNumberOfParts => None,
            Self::InvalidValue => None,
        }
    }
}

impl Display for StrValError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error: ")?;
        Ok(match self {
            Self::UnknownError => write!(f, "an unknown error occurred")?,
            Self::NoType => write!(f, "no type specified")?,
            Self::UnknownType => write!(f, "unknown type specified")?,
            Self::InvalidNumberOfParts => {
                write!(f, "wrong number of parts must be 2 separated by colon (:)")?
            }
            Self::InvalidValue => write!(f, "value is not parseable to type")?,
        })
    }
}

impl From<std::num::ParseIntError> for StrValError {
    fn from(_: std::num::ParseIntError) -> Self {
        StrValError::InvalidValue
    }
}

impl From<()> for StrValError {
    fn from(_: ()) -> Self {
        StrValError::UnknownError
    }
}

pub fn from_string(h: Host, s: &str) -> Result<RawVal, StrValError> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 {
        return Err(StrValError::InvalidNumberOfParts);
    }
    let typ = parts[0];
    let val = parts[1];
    match typ {
        "i32" => Ok(val.parse::<i32>()?.into_val(&h)),
        "u32" => Ok(val.parse::<u32>()?.into_val(&h)),
        "i64" => Ok(val.parse::<i64>()?.into_val(&h)),
        "u64" => Ok(val.parse::<u64>()?.into_val(&h)),
        _ => Err(StrValError::UnknownType),
    }
}

pub fn to_string(h: Host, v: RawVal) -> Result<String, StrValError> {
    if let Ok(v) = i32::try_from_val(&h, v) {
        Ok(format!("i32:{}", v))
    } else if let Ok(v) = u32::try_from_val(&h, v) {
        Ok(format!("u32:{}", v))
    } else if let Ok(v) = i64::try_from_val(&h, v) {
        Ok(format!("i64:{}", v))
    } else if let Ok(v) = u64::try_from_val(&h, v) {
        Ok(format!("u64:{}", v))
    } else {
        Err(StrValError::UnknownType)
    }
}
