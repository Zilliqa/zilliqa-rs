use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;

use crate::Error;

use super::PublicKey;

#[derive(Debug, Clone)]
pub struct PrivateKey(k256::SecretKey);

impl PrivateKey {
    pub fn public_key(&self) -> PublicKey {
        PublicKey::new(self.0.public_key())
    }
}

impl FromStr for PrivateKey {
    type Err = Error;

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

#[cfg(test)]
mod tests {
    use super::PrivateKey;

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
}
