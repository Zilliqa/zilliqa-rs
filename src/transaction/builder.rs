use crate::{crypto::ZilAddress, providers::CreateTransactionRequest, util::parse_zil};

use super::Version;

#[derive(Default, Debug)]
pub struct TransactionParams {
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
    inner_transaction: TransactionParams,
}

impl TransactionBuilder {
    pub fn pay(mut self, amount: u128, to_addr: ZilAddress) -> Self {
        self.inner_transaction.amount = Some(amount);
        self.inner_transaction.to_addr = Some(to_addr);
        self.gas_price_if_none(parse_zil("0.002").unwrap()).gas_limit_if_none(50u64)
    }

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

    pub fn amount_if_none(mut self, amount: u128) -> Self {
        if self.inner_transaction.amount.is_some() {
            return self;
        }

        self.inner_transaction.amount = Some(amount);
        self
    }

    pub fn gas_price(mut self, gas_price: u128) -> Self {
        self.inner_transaction.gas_price = Some(gas_price);
        self
    }

    pub fn gas_price_if_none(mut self, gas_price: u128) -> Self {
        if self.inner_transaction.gas_price.is_some() {
            return self;
        }

        self.inner_transaction.gas_price = Some(gas_price);
        self
    }

    pub fn gas_limit(mut self, gas_limit: u64) -> Self {
        self.inner_transaction.gas_limit = Some(gas_limit);
        self
    }

    pub fn gas_limit_if_none(mut self, gas_limit: u64) -> Self {
        if self.inner_transaction.gas_limit.is_some() {
            return self;
        }
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

    pub fn build(self) -> CreateTransactionRequest {
        CreateTransactionRequest {
            version: self.inner_transaction.version.unwrap_or_default(),
            nonce: self.inner_transaction.nonce.unwrap_or_default(),
            to_addr: self.inner_transaction.to_addr.unwrap_or_default(),
            amount: self.inner_transaction.amount.unwrap_or_default(),
            pub_key: self.inner_transaction.pub_key,
            gas_price: self.inner_transaction.gas_price.unwrap_or_default(),
            gas_limit: self.inner_transaction.gas_limit.unwrap_or_default(),
            code: self.inner_transaction.code,
            data: self.inner_transaction.data,
            signature: self.inner_transaction.signature,
        }
    }
}

impl From<TransactionParams> for TransactionBuilder {
    fn from(value: TransactionParams) -> Self {
        Self {
            inner_transaction: value,
        }
    }
}
