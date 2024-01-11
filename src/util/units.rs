use crate::Error;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Units {
    Zil,
    Li,
    Qa,
    Other(u32),
}

impl FromStr for Units {
    type Err = Error;

    /// Converts a string representation of a unit into an enum variant, returning
    /// an error if the string is not recognized.
    ///
    /// # Example
    /// ```
    /// use zilliqa_rs::util::Units;
    /// let unit: Units = "ZIL".parse().unwrap();
    /// assert_eq!(unit, Units::Zil);
    /// ```
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
    /// Returns the numerical value associated with a given `Units` enum variant.
    /// This value is used in unit conversion.
    pub fn as_num(&self) -> u32 {
        match self {
            Units::Qa => 0,
            Units::Li => 6,
            Units::Zil => 12,
            Units::Other(inner) => *inner,
        }
    }
}
