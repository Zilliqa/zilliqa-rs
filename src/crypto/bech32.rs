use bech32::{FromBase32, ToBase32, Variant};

use crate::{crypto::util::to_checksum_address, util::validation::is_address};

use super::error::CryptoError;

pub fn to_bech32_address(address: &str) -> Result<String, CryptoError> {
    if is_address(address) == false {
        return Err(CryptoError::InvalidAddress(address.to_string()));
    }

    let address = address.replace("0x", "");

    Ok(bech32::encode("zil", hex::decode(address)?.to_base32(), Variant::Bech32).unwrap())
}

pub fn from_bech32_address(address: &str) -> Result<String, CryptoError> {
    let (hrp, data, _) = bech32::decode(address)?;

    let address = hex::encode(Vec::<u8>::from_base32(&data).unwrap());
    println!("{} {}", hrp, address);

    Ok(to_checksum_address(&address)?)
}

#[cfg(test)]
mod tests {
    use crate::crypto::bech32::{from_bech32_address, to_bech32_address};

    #[test]
    fn to_bech32_address_should_return_correct_address() {
        let address = "0x381f4008505e940AD7681EC3468a719060caF796";
        let bech32_address = "zil18q05qzzst62q44mgrmp5dzn3jpsv4aukxredu2";

        assert_eq!(bech32_address, to_bech32_address(&address).unwrap())
    }

    #[test]
    fn from_bech32_address_should_return_correct_address() {
        let bech32_address = "zil18q05qzzst62q44mgrmp5dzn3jpsv4aukxredu2";
        let expected = "0x381f4008505e940AD7681EC3468a719060caF796";
        assert_eq!(expected, from_bech32_address(bech32_address).unwrap());
    }
}
