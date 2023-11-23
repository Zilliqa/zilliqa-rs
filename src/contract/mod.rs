pub mod error;
pub mod factory;

#[derive(Debug)]
pub struct Contract {
    pub address: ZilAddress,
}

pub use factory::Factory as ContractFactory;
use serde::Serialize;

use crate::crypto::ZilAddress;

#[derive(Debug, Serialize)]
pub struct Init(pub Vec<Value>);

#[derive(Debug, Serialize)]
pub struct Value {
    vname: String,

    #[serde(rename = "type")]
    r#type: String,
    value: String,
}

impl Value {
    pub fn new(vname: &str, type_: &str, value: &str) -> Self {
        Self {
            vname: vname.to_string(),
            value: value.to_string(),
            r#type: type_.to_string(),
        }
    }
}
