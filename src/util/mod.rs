pub mod units;
pub mod validation;
use crate::Error;
pub use units::*;

use ethers_core::utils::{format_units as eth_format_units, parse_units as eth_parse_units, ParseUnits};
use primitive_types::U256;

pub fn parse_units<K, S>(amount: S, units: K) -> Result<ParseUnits, Error>
where
    S: ToString,
    K: TryInto<Units, Error = Error> + Copy,
{
    let exponent: u32 = units.try_into()?.as_num();
    eth_parse_units(amount, exponent).map_err(Error::EthersConversionError)
}

pub fn format_units<T, K>(amount: T, units: K) -> Result<String, Error>
where
    T: Into<ParseUnits>,
    K: TryInto<Units, Error = Error>,
{
    let exponent: u32 = units.try_into()?.as_num();
    eth_format_units(amount, exponent).map_err(Error::EthersConversionError)
}

pub fn format_zil<T: Into<ParseUnits>>(amount: T) -> String {
    format_units(amount, "zil").unwrap()
}

pub fn parse_zil<S: ToString>(zil: S) -> Result<U256, Error> {
    Ok(parse_units(zil, "zil")?.into())
}

#[cfg(test)]
mod tests {
    use primitive_types::U256;

    use super::{format_units, format_zil, parse_units, parse_zil};

    #[test]
    fn parse_units_should_work() {
        let amount_in_zil = U256::from_dec_str("15230001000000").unwrap();
        let amount_in_li = U256::from_dec_str("15230001").unwrap();
        let amount_in_qa = U256::from_dec_str("15").unwrap();
        assert_eq!(amount_in_zil, parse_units("15.230001000000000000", "zil").unwrap().into());
        assert_eq!(amount_in_li, parse_units("15.230001000000000000", "li").unwrap().into());
        assert_eq!(amount_in_qa, parse_units("15.230001000000000000", "qa").unwrap().into());
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
        let amount_in_zil = U256::from_dec_str("15230001000000").unwrap();
        assert_eq!(amount_in_zil, parse_zil("15.230001000000000000").unwrap().into());
    }

    #[test]
    fn format_zil_should_work() {
        let zil = format_zil(1395633240123_u128);
        assert_eq!(zil, "1.395633240123");
    }
}
