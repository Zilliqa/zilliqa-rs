use bech32::{FromBase32, ToBase32, Variant};
use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer};
use sha2::Digest;
use std::{fmt::Display, ops::Deref, str::FromStr};

use crate::{
    util::validation::{is_address, is_bech32},
    Error,
};

use super::{to_checksum_address, PublicKey};

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

    fn try_from(value: &PublicKey) -> Result<Self, Self::Error> {
        let mut hasher = sha2::Sha256::new();
        hasher.update(value.to_sec1_bytes());
        Ok(Self(to_checksum_address(&hex::encode(hasher.finalize())[24..])?))
    }
}

impl ZilAddress {
    fn from_bech32(address: &str) -> Result<Self, Error> {
        let (_hrp, data, _) = bech32::decode(address)?;

        let address = hex::encode(Vec::<u8>::from_base32(&data)?);

        Ok(Self(to_checksum_address(&address)?))
    }

    pub fn to_bech32(&self) -> Result<String, Error> {
        let address = self.0.strip_prefix("0x").unwrap(); // Safe to call unwrap, we create addresses with 0x prefixed

        Ok(bech32::encode("zil", hex::decode(address)?.to_base32(), Variant::Bech32)?)
    }

    pub fn nil() -> Self {
        Self("0x0000000000000000000000000000000000000000".to_string())
    }
}

impl FromStr for ZilAddress {
    type Err = Error;

    fn from_str(addr: &str) -> Result<Self, Self::Err> {
        if is_address(addr) {
            Ok(Self(to_checksum_address(addr)?))
        } else if is_bech32(addr) {
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

    use crate::crypto::ZilAddress;

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
}
