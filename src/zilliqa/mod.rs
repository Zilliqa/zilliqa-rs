pub mod error;

use std::{cell::RefCell, rc::Rc};

use crate::{account::wallet::Wallet, blockchain::chain::Blockchain, core::HTTPProvider};

use self::error::ZilliqaError;

pub struct Zilliqa {
    pub chain_id: u16,
    pub provider: Rc<HTTPProvider>,
    pub blockchain: Blockchain,
    pub wallet: Rc<RefCell<Wallet>>,
}

impl Zilliqa {
    pub fn new(url: &str, chain_id: u16) -> Result<Self, ZilliqaError> {
        let provider = Rc::new(HTTPProvider::new(url)?);
        let wallet = Rc::new(RefCell::new(Wallet::new(provider.clone())));
        let blockchain = Blockchain::new(provider.clone(), wallet.clone(), chain_id);

        Ok(Self {
            chain_id,
            provider,
            blockchain,
            wallet,
        })
    }
}
