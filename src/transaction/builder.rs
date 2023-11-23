use crate::crypto::ZilAddress;

use super::{Transaction, Version};

#[derive(Default, Debug)]
pub struct TransactionRequest {
    pub version: Option<Version>,
    pub nonce: Option<u64>,
    pub to_addr: Option<ZilAddress>,
    pub amount: Option<u128>,
    pub pub_key: Option<String>,
    pub gas_price: Option<u128>,
    pub gas_limit: Option<u64>,
    pub code: Option<String>,
    pub data: Option<String>,
    pub signature: Option<String>,
}

#[derive(Default, Debug)]
pub struct TransactionBuilder {
    inner_transaction: TransactionRequest,
}

impl TransactionBuilder {
    pub fn chain_id(mut self, chain_id: u16) -> Self {
        self.inner_transaction.version = Some(Version::new(chain_id));
        self
    }

    pub fn nonce(mut self, nonce: u64) -> Self {
        self.inner_transaction.nonce = Some(nonce);
        self
    }

    pub fn to_address(mut self, to_addr: ZilAddress) -> Self {
        self.inner_transaction.to_addr = Some(to_addr);
        self
    }

    pub fn amount(mut self, amount: u128) -> Self {
        self.inner_transaction.amount = Some(amount);
        self
    }

    pub fn gas_price(mut self, gas_price: u128) -> Self {
        self.inner_transaction.gas_price = Some(gas_price);
        self
    }

    pub fn gas_limit(mut self, gas_limit: u64) -> Self {
        self.inner_transaction.gas_limit = Some(gas_limit);
        self
    }

    pub fn pub_key(mut self, pub_key: String) -> Self {
        self.inner_transaction.pub_key = Some(pub_key);
        self
    }

    pub fn data(mut self, data: String) -> Self {
        self.inner_transaction.data = Some(data);
        self
    }

    pub fn code(mut self, code: String) -> Self {
        self.inner_transaction.code = Some(code);
        self
    }

    pub fn signature(mut self, signature: String) -> Self {
        self.inner_transaction.signature = Some(signature);
        self
    }

    pub fn build(self) -> Transaction {
        self.inner_transaction.into()
    }
}
