use std::fmt::Display;

use serde::{Serialize, Serializer};

/// The `Version` struct represents a version with a message version and a chain ID.
///
/// Properties:
///
/// * `msg_version`: The `msg_version` property represents the version of the message being sent or
/// received.
/// * `chain_id`: The `chain_id` property represents the identifier of a blockchain network. It is
/// typically used to differentiate between different blockchain networks, such as the mainnet, testnet,
/// or any other custom networks.
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Version {
    msg_version: u16,
    chain_id: u16,
}

impl Version {
    pub fn new(chain_id: u16) -> Self {
        Self {
            chain_id,
            msg_version: 1,
        }
    }

    /// The `pack` function takes the `chain_id` and `msg_version` values and packs them into a
    /// single `u32` value.
    pub fn pack(&self) -> u32 {
        (self.chain_id as u32) << 16 | (self.msg_version as u32)
    }

    pub fn valid(&self) -> bool {
        (self.chain_id > 0) && (self.msg_version > 0)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "chain_id: {}, msg_version: {}", self.chain_id, self.msg_version)
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let packed = self.pack();
        serializer.serialize_u32(packed)
    }
}

#[cfg(test)]
mod tests {
    use super::Version;

    #[test]
    fn pack_should_work_fine() {
        let version = Version::new(16);
        assert_eq!(version.pack(), 0x0010_0001)
    }
}
