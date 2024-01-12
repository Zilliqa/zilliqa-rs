use std::{fmt::Display, str::FromStr};

use serde::Deserialize;

use crate::{util::validation::is_byte_string, Error};

#[derive(Debug, Clone, Deserialize)]
pub struct TxHash(String);

impl TxHash {
    pub fn is_valid(tx_hash: &str) -> bool {
        is_byte_string(tx_hash, 64)
    }
}

impl FromStr for TxHash {
    type Err = Error;

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

#[cfg(test)]
mod tests {
    use super::TxHash;

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
}
