use std::fmt::Display;

use serde::Serializer;

#[derive(serde::Serialize, Default, Debug)]
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
pub struct TransactionRequest {
    pub version: Option<u32>,
    pub nonce: Option<u64>,
    pub to_addr: Option<String>,
    pub amount: Option<u128>,
    pub pub_key: Option<String>,
    pub gas_price: Option<u128>,
    pub gas_limit: Option<u64>,
    pub code: Option<String>,
    pub data: Option<String>,
    pub signature: Option<String>,
}

#[derive(Default)]
pub struct TransactionBuilder {
    inner_transaction: TransactionRequest,
}

impl From<TransactionRequest> for Transaction {
    fn from(val: TransactionRequest) -> Self {
        Self {
            version: val.version.unwrap_or_default(),
            nonce: val.nonce.unwrap_or_default(),
            to_addr: val.to_addr.unwrap_or_default(),
            amount: val.amount.unwrap_or_default(),
            pub_key: val.pub_key,
            gas_price: val.gas_price.unwrap_or_default(),
            gas_limit: val.gas_limit.unwrap_or_default(),
            code: val.code.unwrap_or_default(),
            data: val.data.unwrap_or_default(),
            signature: val.signature,
        }
    }
}

impl TransactionBuilder {
    pub fn version(mut self, version: u32) -> Self {
        self.inner_transaction.version = Some(version);
        self
    }

    pub fn nonce(mut self, nonce: u64) -> Self {
        self.inner_transaction.nonce = Some(nonce);
        self
    }

    pub fn to_address(mut self, to_addr: &str) -> Self {
        self.inner_transaction.to_addr = Some(to_addr.to_string());
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

impl From<TransactionBuilder> for Transaction {
    fn from(value: TransactionBuilder) -> Self {
        value.inner_transaction.into()
    }
}
