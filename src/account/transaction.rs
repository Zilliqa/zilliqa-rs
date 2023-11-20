use std::fmt::Display;

use serde::Serializer;

#[derive(serde::Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub version: u32,
    pub nonce: u64,
    pub to_addr: String,
    #[serde(serialize_with = "to_str")]
    pub amount: u128,
    pub pub_key: Option<String>,
    #[serde(serialize_with = "to_str")]
    pub gas_price: u128,
    #[serde(serialize_with = "to_str")]
    pub gas_limit: u64,
    pub code: String,
    pub data: String,
    pub signature: Option<String>,
}

pub fn to_str<S: Serializer, T: Display>(data: T, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&data.to_string())
}

#[derive(Default)]
pub struct TransactionBuilder {
    inner_transaction: Transaction,
}

impl TransactionBuilder {
    pub fn version(mut self, version: u32) -> Self {
        self.inner_transaction.version = version;
        self
    }

    pub fn nonce(mut self, nonce: u64) -> Self {
        self.inner_transaction.nonce = nonce;
        self
    }

    pub fn to_address(mut self, to_addr: &str) -> Self {
        self.inner_transaction.to_addr = to_addr.to_string();
        self
    }

    pub fn amount(mut self, amount: u128) -> Self {
        self.inner_transaction.amount = amount;
        self
    }

    pub fn gas_price(mut self, gas_price: u128) -> Self {
        self.inner_transaction.gas_price = gas_price;
        self
    }

    pub fn gas_limit(mut self, gas_limit: u64) -> Self {
        self.inner_transaction.gas_limit = gas_limit;
        self
    }

    pub fn pub_key(mut self, pub_key: String) -> Self {
        self.inner_transaction.pub_key = Some(pub_key);
        self
    }

    pub fn data(mut self, data: String) -> Self {
        self.inner_transaction.data = data;
        self
    }

    pub fn code(mut self, code: String) -> Self {
        self.inner_transaction.code = code;
        self
    }

    pub fn signature(mut self, signature: String) -> Self {
        self.inner_transaction.signature = Some(signature);
        self
    }

    pub fn build(self) -> Transaction {
        self.inner_transaction
    }
}
