use std::fmt::Display;

use serde::Serializer;

// TODO: DRY
pub struct TransactionRequest {
    pub nonce: Option<u64>,
    pub to_addr: String,
    pub amount: Option<u128>,
    pub pub_key: Option<String>,
    pub gas_price: u128,
    pub gas_limit: u64,
    pub code: Option<String>,
    pub data: Option<String>,
}

#[derive(serde::Serialize)]
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
