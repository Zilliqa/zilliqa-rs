use bech32::{ToBase32, Variant};

use crate::util::validation::is_address;

use super::error::CryptoError;

pub fn to_bech32_address(address: &str) -> Result<String, CryptoError> {
    if is_address(address) == false {
        return Err(CryptoError::InvalidAddress(address.to_string()));
    }

    let address = address.replace("0x", "");

    Ok(bech32::encode("zil", hex::decode(address)?.to_base32(), Variant::Bech32).unwrap())
}

#[cfg(test)]
mod tests {
    use crate::crypto::bech32::to_bech32_address;

    #[test]
    fn to_bech32_address_should_return_correct_address() {
        let address = "0x381f4008505e940AD7681EC3468a719060caF796";
        let bech32_address = "zil18q05qzzst62q44mgrmp5dzn3jpsv4aukxredu2";

        assert_eq!(bech32_address, to_bech32_address(&address).unwrap())
    }
}
