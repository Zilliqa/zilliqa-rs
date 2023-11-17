pub mod error;

use crate::{blockchain::chain::Blockchain, core::HTTPProvider};

use self::error::ZilliqaError;

pub struct Zilliqa {
    pub blockchain: Blockchain,
}

impl Zilliqa {
    pub fn new(url: &str) -> Result<Self, ZilliqaError> {
        let provider = HTTPProvider::new(url)?;
        let blockchain = Blockchain::new(provider);

        Ok(Self { blockchain })
    }
}
