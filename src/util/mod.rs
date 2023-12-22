pub mod units;
pub mod validation;
use crate::Error;
pub use units::*;

pub fn parse_units<K, S>(amount: S, units: K) -> Result<u128, Error>
where
    S: ToString,
    K: TryInto<Units, Error = Error> + Copy,
{
    let exponent: u32 = units.try_into()?.as_num();
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

pub fn format_units<K>(amount: u128, units: K) -> Result<String, Error>
where
    K: TryInto<Units, Error = Error>,
{
    let units = units.try_into()?.as_num();
    let exp10 = 10_u128.pow(units);
    let integer = amount / exp10;
    let decimals = (amount % exp10).to_string();
    let units = units as usize;
    Ok(format!("{integer}.{decimals:0>units$}"))
}

/// The function `format_zil` formats a given amount of zil into a string
/// representation.
///
/// Arguments:
///
/// * `amount`: The `amount` is used to specify the amount in QA unit to be formatted.
///
/// Returns:
///
/// The `format_zil` function returns a `String` that represents the formatted amount of ZIL.
pub fn format_zil(amount: u128) -> String {
    // Safe to call unwrap, "zil" can be converted to unit
    format_units(amount, "zil").unwrap()
}

/// The `parse_zil` function is a convenience function that parses a string representation of an amount
/// in ZIL into a QA.
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
