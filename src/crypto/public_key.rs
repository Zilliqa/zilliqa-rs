use std::{fmt::Display, ops::Deref, str::FromStr};

use super::error::CryptoError;

pub struct PublicKey(k256::PublicKey);

impl PublicKey {
    pub fn new(pk: k256::PublicKey) -> Self {
        Self(pk)
    }
}

impl FromStr for PublicKey {
    type Err = CryptoError;

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

#[cfg(test)]
mod tests {
    use super::PublicKey;

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

    // TODO: Add more tests
}
