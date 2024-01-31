//! Shared data types and functionalities.

#[doc(hidden)]
pub mod proto;
pub mod types;
pub mod units;

use bech32::{FromBase32, ToBase32, Variant};
pub use types::*;
pub use units::*;

use std::{
    fmt::Display,
    ops::{BitAnd, Deref},
    str::FromStr,
};

use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer, Serialize};
use sha2::Digest;

use crate::Error;

/// Checks if a given string is a valid byte string of a specified length.
fn is_byte_string(str: &str, len: usize) -> bool {
    let regex = regex::Regex::new(&format!("^[0-9a-fA-F]{{{}}}$", len)).expect("Failed to create the regex for `is_byte_string`");
    let str = str.replace("0x", "");
    regex.is_match(&str)
}

/// A Type-safe Transaction hash.
#[derive(Debug, Clone, Deserialize)]
pub struct TxHash(String);

impl TxHash {
    /// Checks if a given transaction hash is valid.
    ///
    /// Example
    /// ```
    /// use zilliqa_rs::core::TxHash;
    /// assert!(TxHash::is_valid("bdadfd994f452df803cc223d1f417b02830ac96dbe5edad1b9f8d58613f95206"));
    /// ```
    pub fn is_valid(tx_hash: &str) -> bool {
        is_byte_string(tx_hash, 64)
    }
}

impl FromStr for TxHash {
    type Err = Error;
    /// Parses a given string slice to Transaction hash.
    ///
    /// # Example
    /// ```
    /// use zilliqa_rs::core::TxHash;
    /// let hash: TxHash = "bdadfd994f452df803cc223d1f417b02830ac96dbe5edad1b9f8d58613f95206".parse().unwrap();
    /// ```
    fn from_str(tx_hash: &str) -> Result<Self, Self::Err> {
        if TxHash::is_valid(tx_hash) {
            Ok(Self(tx_hash.to_string()))
        } else {
            Err(Error::InvalidTransactionHash(tx_hash.to_string()))
        }
    }
}

impl Display for TxHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Rust corresponding type for scilla BNum.
#[derive(Serialize, Debug, Clone, Deserialize, PartialEq)]
pub struct BNum(String);
impl BNum {
    pub fn new(bnum: &str) -> Self {
        Self(bnum.to_string())
    }
}

impl Deref for BNum {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for BNum {
    type Err = Error;

    // TODO: Make to more strict.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}

#[derive(Debug, Clone)]
/// secp256k1 (K-256) secret key.
pub struct PrivateKey(k256::SecretKey);

impl PrivateKey {
    /// Generates a random private key.
    ///
    /// # Example
    /// ```
    /// use zilliqa_rs::core::PrivateKey;
    /// let private_key = PrivateKey::create_random();
    /// ```
    pub fn create_random() -> Self {
        Self(k256::SecretKey::random(&mut rand::thread_rng()))
    }

    /// Constructs a private key from a raw secret key.
    pub fn from_slice(slice: &[u8]) -> Result<Self, Error> {
        Ok(Self(k256::SecretKey::from_slice(slice)?))
    }

    /// Returns corresponding public key of the private key
    pub fn public_key(&self) -> PublicKey {
        PublicKey::new(self.0.public_key())
    }
}

impl FromStr for PrivateKey {
    type Err = Error;

    /// Create a private key out of a sting slice.
    ///
    /// # Example
    /// ```
    /// use zilliqa_rs::core::PrivateKey;
    ///let pv: PrivateKey = "D96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba"
    ///    .parse()
    ///    .unwrap();
    ///assert_eq!(
    ///    "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba",
    ///    pv.to_string()
    ///);
    ///```
    fn from_str(secret_key: &str) -> Result<Self, Self::Err> {
        let secret_key = match secret_key.strip_prefix("0x") {
            Some(secret_key) => secret_key,
            None => secret_key,
        };
        Ok(Self(k256::SecretKey::from_slice(&hex::decode(secret_key)?)?))
    }
}

impl Deref for PrivateKey {
    type Target = k256::SecretKey;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for PrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.to_bytes()).to_lowercase())
    }
}

impl PartialEq for PrivateKey {
    // TODO: Make it efficient
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

/// secp256k1 (K-256) public key.
#[derive(Debug, Clone)]
pub struct PublicKey(k256::PublicKey);

impl PublicKey {
    /// Creates a new public key.
    pub fn new(pk: k256::PublicKey) -> Self {
        Self(pk)
    }
}

impl FromStr for PublicKey {
    type Err = Error;

    /// Parse a string into a public key
    ///
    /// # Example
    ///  ```
    /// use zilliqa_rs::core::PublicKey;
    /// let public_key: PublicKey = "03bfad0f0b53cff5213b5947f3ddd66acee8906aba3610c111915aecc84092e052"
    ///     .parse()
    ///     .unwrap();
    /// assert_eq!(
    ///     public_key.to_string(),
    ///     "03bfad0f0b53cff5213b5947f3ddd66acee8906aba3610c111915aecc84092e052"
    /// );
    /// ```
    fn from_str(public_key: &str) -> Result<Self, Self::Err> {
        let public_key = match public_key.strip_prefix("0x") {
            Some(public_key) => public_key,
            None => public_key,
        };

        Ok(Self(k256::PublicKey::from_sec1_bytes(&hex::decode(public_key)?)?))
    }
}

impl PartialEq for PublicKey {
    // TODO: Make it efficient
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Deref for PublicKey {
    type Target = k256::PublicKey;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.to_sec1_bytes()).to_lowercase())
    }
}

/// Type-safe address fo zilliqa network.
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
    /// use zilliqa_rs::core::ZilAddress;
    ///
    /// assert!(ZilAddress::is_bech32("zil18q05qzzst62q44mgrmp5dzn3jpsv4aukxredu2"))
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

    use super::{is_byte_string, PrivateKey, PublicKey, TxHash, ZilAddress};

    #[test]
    fn is_byte_string_should_return_true_for_a_valid_byte_string_with_correct_size() {
        let str = "1234567890";
        assert!(is_byte_string(str, str.len()))
    }

    #[test]
    fn is_byte_string_should_return_true_for_a_valid_byte_string_with_correct_size_even_if_its_prepended_with_0x() {
        let str = "0x1234567890";
        assert!(is_byte_string(str, str.len() - 2)) // -2 for 0x
    }

    #[test]
    fn is_byte_string_should_return_true_for_a_valid_byte_string_with_correct_size_when_it_contains_letters_a_f() {
        let str = "1234567890aabbccddeeff";
        assert!(is_byte_string(str, str.len()))
    }

    #[test]
    fn is_byte_string_should_return_false_if_size_is_incorrect() {
        let str = "1234567890aabbccddeeff";
        assert_eq!(is_byte_string(str, str.len() - 2), false);
    }

    #[test]
    fn is_byte_string_should_return_false_if_contains_out_of_a_f_characters() {
        let str = "1234567890aabbccddeeffgg";
        assert_eq!(is_byte_string(str, str.len()), false);
    }

    #[test]
    fn is_tx_hash_should_return_true_for_a_valid_hash() {
        let hash = "bdadfd994f452df803cc223d1f417b02830ac96dbe5edad1b9f8d58613f95206";
        assert!(TxHash::is_valid(hash));

        let hash = "bdadfd994f452df803cc223d1f417b02830ac96dbe5edad1b9f8d58613f95206".to_ascii_uppercase();
        assert!(TxHash::is_valid(&hash));
    }

    #[test]
    fn is_tx_hash_should_return_false_for_a_invalid_hash() {
        let hash = "bdadfd994f452df803cc223d102830ac96dbe5edad1b9f8d58613f95206";
        assert!(!TxHash::is_valid(hash));
    }

    #[test]
    fn should_parse_hex_string_to_private_key() {
        let pv: PrivateKey = "D96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba"
            .parse()
            .unwrap();
        assert_eq!(
            "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba",
            pv.to_string()
        );

        assert_eq!(
            pv.public_key().to_string(),
            "03bfad0f0b53cff5213b5947f3ddd66acee8906aba3610c111915aecc84092e052"
        );
    }

    #[test]
    fn should_parse_hex_string_to_private_key_if_prefixed_with_0x() {
        let pv: PrivateKey = "0xD96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba"
            .parse()
            .unwrap();
        assert_eq!(
            "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba",
            pv.to_string()
        );

        assert_eq!(
            pv.public_key().to_string(),
            "03bfad0f0b53cff5213b5947f3ddd66acee8906aba3610c111915aecc84092e052"
        );
    }

    #[test]
    fn should_parse_public_key_from_hexstring() {
        let public_key: PublicKey = "03bfad0f0b53cff5213b5947f3ddd66acee8906aba3610c111915aecc84092e052"
            .parse()
            .unwrap();

        assert_eq!(
            public_key.to_string(),
            "03bfad0f0b53cff5213b5947f3ddd66acee8906aba3610c111915aecc84092e052"
        );
    }

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
