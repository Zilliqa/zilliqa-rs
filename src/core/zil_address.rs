use bech32::{FromBase32, ToBase32, Variant};
use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer};
use sha2::Digest;
use std::ops::BitAnd;
use std::{fmt::Display, ops::Deref, str::FromStr};

use crate::util::is_byte_string;
use crate::Error;

use super::PublicKey;

#[derive(Eq, Hash, Debug, PartialEq, Clone, serde::Serialize, Default)]
pub struct ZilAddress(String);

impl Deref for ZilAddress {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&PublicKey> for ZilAddress {
    type Error = Error;

    /// Convert a public key into a ZilAddress
    fn try_from(value: &PublicKey) -> Result<Self, Self::Error> {
        let mut hasher = sha2::Sha256::new();
        hasher.update(value.to_sec1_bytes());
        Ok(Self(ZilAddress::to_checksum_address(&hex::encode(hasher.finalize())[24..])?))
    }
}

impl ZilAddress {
    fn from_bech32(address: &str) -> Result<Self, Error> {
        let (_hrp, data, _) = bech32::decode(address)?;

        let address = hex::encode(Vec::<u8>::from_base32(&data)?);

        Ok(Self(ZilAddress::to_checksum_address(&address)?))
    }

    fn to_checksum_address(address: &str) -> Result<String, Error> {
        let address = address.replace("0x", "");
        if !ZilAddress::is_address(&address) {
            return Err(Error::InvalidAddress(address.to_string()));
        }

        let mut hasher = sha2::Sha256::new();
        hasher.update(hex::decode(&address)?);
        let v = primitive_types::U256::from_big_endian(&hasher.finalize());
        let ret = address
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if c.is_ascii_digit() {
                    c
                } else {
                    let cond = v
                        .bitand(primitive_types::U256::from(2).pow(primitive_types::U256::from(255 - 6 * i)))
                        .ge(&primitive_types::U256::one());
                    if cond {
                        c.to_ascii_uppercase()
                    } else {
                        c.to_ascii_lowercase()
                    }
                }
            })
            .collect::<String>();

        Ok(format!("0x{}", ret))
    }

    pub fn to_bech32(&self) -> Result<String, Error> {
        let address = self.0.strip_prefix("0x").unwrap(); // Safe to call unwrap, we create addresses with 0x prefixed

        Ok(bech32::encode("zil", hex::decode(address)?.to_base32(), Variant::Bech32)?)
    }

    /// Create an empty ZilAddress, mainly to deploy a new contract.
    pub fn nil() -> Self {
        Self("0x0000000000000000000000000000000000000000".to_string())
    }

    /// Checks if the given raw string slice is a valid bech32 address.
    ///
    /// # Example
    /// ```
    /// assert!(is_bech32("zil18q05qzzst62q44mgrmp5dzn3jpsv4aukxredu2"))
    /// ```
    pub fn is_bech32(raw: &str) -> bool {
        let regex = regex::Regex::new("^zil1[qpzry9x8gf2tvdw0s3jn54khce6mua7l]{38}$")
            .expect("Failed to create the regex for `is_bech32`");

        regex.is_match(raw)
    }

    pub fn is_address(address: &str) -> bool {
        is_byte_string(address, 40)
    }
}

impl FromStr for ZilAddress {
    type Err = Error;

    /// Parse a string slice into a ZilAddress.
    fn from_str(addr: &str) -> Result<Self, Self::Err> {
        if ZilAddress::is_address(addr) {
            Ok(Self(ZilAddress::to_checksum_address(addr)?))
        } else if ZilAddress::is_bech32(addr) {
            Self::from_bech32(addr)
        } else {
            Err(Error::InvalidAddress(addr.to_string()))
        }
    }
}

impl Display for ZilAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for ZilAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;

        s.parse::<Self>().map_err(D::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use claim::assert_ok;

    use crate::core::ZilAddress;

    #[test]
    fn valid_address_should_parse_correctly() {
        let address = "0x381f4008505e940AD7681EC3468a719060caF796";
        assert_ok!(address.parse::<ZilAddress>());
        assert_eq!(address.parse::<ZilAddress>().unwrap().to_string(), address);

        assert_ok!(address.strip_prefix("0x").unwrap().parse::<ZilAddress>());
        assert_eq!(
            address.strip_prefix("0x").unwrap().parse::<ZilAddress>().unwrap().to_string(),
            address
        );
    }

    #[test]
    fn valid_bech32_address_should_parse_correctly() {
        let address = "0x381f4008505e940AD7681EC3468a719060caF796";
        let bech32_address = "zil18q05qzzst62q44mgrmp5dzn3jpsv4aukxredu2";
        let zil_addr: ZilAddress = bech32_address.parse().unwrap();

        assert_eq!(zil_addr.to_string(), address);
    }

    #[test]
    fn to_bech32_address_should_return_correct_address() {
        let address = "0x381f4008505e940AD7681EC3468a719060caF796";
        let bech32_address = "zil18q05qzzst62q44mgrmp5dzn3jpsv4aukxredu2";

        let zil_addr: ZilAddress = address.parse().unwrap();
        assert_eq!(zil_addr.to_bech32().unwrap(), bech32_address);
    }

    #[test]
    fn is_bech32_should_return_true_for_valid_one() {
        assert!(ZilAddress::is_bech32("zil18q05qzzst62q44mgrmp5dzn3jpsv4aukxredu2"))
    }

    #[test]
    fn is_bech32_should_return_false_for_invalid_ones() {
        assert!(!ZilAddress::is_bech32("liz18q05qzzst62q44mgrmp5dzn3jpsv4aukxredu2"));
        assert!(!ZilAddress::is_bech32("zil18q05qzzst62q44mgrmp5dzn3jpsv4aukxredu2ssaas"));
    }

    #[test]
    fn to_checksum_address_should_return_correct_value_for_valid_input() {
        let address = "11223344556677889900aabbccddeeff11223344";
        let checksum = "0x11223344556677889900AabbccdDeefF11223344";

        assert_eq!(checksum, ZilAddress::to_checksum_address(address).unwrap())
    }
}
