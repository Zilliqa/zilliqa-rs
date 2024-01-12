//! Utility functions for unit conversion.

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

/// Converts given amount with given unit to Qa.
/// # Example
/// ```
/// use zilliqa_rs::util::parse_units;
/// let amount_in_zil = 15230001000000_u128;
/// let amount_in_li = 15230001_u128;
/// let amount_in_qa = 15_u128;
/// assert_eq!(amount_in_zil, parse_units("15.230001000000000000", "zil").unwrap());
/// assert_eq!(amount_in_li, parse_units("15.230001000000000000", "li").unwrap());
/// assert_eq!(amount_in_qa, parse_units("15.230001000000000000", "qa").unwrap());
/// ```
pub fn parse_units<K, S>(amount: S, unit: K) -> Result<u128, Error>
where
    S: ToString,
    K: TryInto<Units, Error = Error> + Copy,
{
    let exponent: u32 = unit.try_into()?.as_num();
    let mut amount_str = amount.to_string().replace('_', "");
    let negative = amount_str.chars().next().unwrap_or_default() == '-';
    let dec_len = if let Some(di) = amount_str.find('.') {
        amount_str.remove(di);
        amount_str[di..].len() as u32
    } else {
        0
    };

    if dec_len > exponent {
        // Truncate the decimal part if it is longer than the exponent
        let amount_str = &amount_str[..(amount_str.len() - (dec_len - exponent) as usize)];
        if negative {
            Err(Error::NegativeValueNotAllowed)
        } else {
            Ok(amount_str.parse::<u128>()?)
        }
    } else if negative {
        Err(Error::NegativeValueNotAllowed)
    } else {
        let mut a_uint: u128 = amount_str.parse()?;
        a_uint *= 10_u128.checked_pow(exponent - dec_len).ok_or(Error::ParseOverflow)?;
        Ok(a_uint)
    }
}

/// Converts given amount in Qa to requested unit.
/// # Example
///
///```
/// use zilliqa_rs::util::format_units;
///
///let zil = format_units(1395633240123_u128, "zil").unwrap();
///assert_eq!(zil, "1.395633240123");
///
///let li = format_units(1395633240123_u128, "li").unwrap();
///assert_eq!(li, "1395633.240123");
///```
///
pub fn format_units<K>(amount: u128, unit: K) -> Result<String, Error>
where
    K: TryInto<Units, Error = Error>,
{
    let units = unit.try_into()?.as_num();
    let exp10 = 10_u128.pow(units);
    let integer = amount / exp10;
    let decimals = (amount % exp10).to_string();
    let units = units as usize;
    Ok(format!("{integer}.{decimals:0>units$}"))
}

/// Formats a given amount in Qa into Zil.
///
/// # Example
/// ```
/// use zilliqa_rs::util::format_zil;
///
/// let zil = format_zil(1395633240123_u128);
/// assert_eq!(zil, "1.395633240123");
/// ```
pub fn format_zil(amount: u128) -> String {
    // Safe to call unwrap, "zil" can be converted to unit
    format_units(amount, "zil").unwrap()
}

/// Parses an amount in ZIL into a QA.
///
/// # Example
/// ```
/// use zilliqa_rs::core::parse_zil;
///
/// let amount_in_qa = 15230001000000;
/// assert_eq!(amount_in_qa, parse_zil("15.230001").unwrap());
/// ```
pub fn parse_zil<S: ToString>(zil: S) -> Result<u128, Error> {
    parse_units(zil, "zil")
}

#[cfg(test)]
mod tests {
    use claim::assert_err;

    use super::{format_units, format_zil, parse_units, parse_zil};

    #[test]
    fn parse_units_should_work() {
        let amount_in_zil = 15230001000000_u128;
        let amount_in_li = 15230001_u128;
        let amount_in_qa = 15_u128;
        assert_eq!(amount_in_zil, parse_units("15.230001000000000000", "zil").unwrap());
        assert_eq!(amount_in_li, parse_units("15.230001000000000000", "li").unwrap());
        assert_eq!(amount_in_qa, parse_units("15.230001000000000000", "qa").unwrap());
    }

    #[test]
    fn parse_units_should_return_error_for_negative_values() {
        assert_err!(parse_units("-15.230001000000000000", "zil"));
    }

    #[test]
    fn format_units_should_work() {
        let zil = format_units(1395633240123_u128, "zil").unwrap();
        assert_eq!(zil, "1.395633240123");

        let li = format_units(1395633240123_u128, "li").unwrap();
        assert_eq!(li, "1395633.240123");
    }

    #[test]
    fn parse_zil_should_work() {
        let amount_in_zil = 15230001000000;
        assert_eq!(amount_in_zil, parse_zil("15.230001000000000000").unwrap());
    }

    #[test]
    fn parse_zil_should_error_for_negative_values() {
        assert_err!(parse_zil("-15.230001000000000000"));
    }

    #[test]
    fn format_zil_should_work() {
        let zil = format_zil(1395633240123_u128);
        assert_eq!(zil, "1.395633240123");
    }
}
