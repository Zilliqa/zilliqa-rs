use crate::Error;
use std::str::FromStr;

pub enum Units {
    Zil,
    Li,
    Qa,
    Other(u32),
}

impl FromStr for Units {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "zil" | "ZIL" => Self::Zil,
            "qa" | "QA" => Self::Qa,
            "li" | "LI" => Self::Li,
            _ => return Err(Error::UnrecognizedUnits(s.to_string())),
        })
    }
}

impl TryFrom<&str> for Units {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl Units {
    pub fn as_num(&self) -> u32 {
        match self {
            Units::Qa => 0,
            Units::Li => 6,
            Units::Zil => 12,
            Units::Other(inner) => *inner,
        }
    }
}
