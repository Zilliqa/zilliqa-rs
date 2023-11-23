pub mod builder;
pub mod version;

pub use builder::*;
pub use version::*;

use std::fmt::Display;

use serde::Serializer;

use crate::crypto::ZilAddress;

#[derive(serde::Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub version: Version,
    pub nonce: u64,
    pub to_addr: ZilAddress,
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
