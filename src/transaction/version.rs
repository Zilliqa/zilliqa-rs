use std::fmt::Display;

use serde::{Serialize, Serializer};

#[derive(Debug, Default, PartialEq)]
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
