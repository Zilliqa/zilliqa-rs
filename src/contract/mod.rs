pub mod error;
pub mod factory;

pub struct Contract;
pub use factory::Factory as ContractFactory;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Init(pub Vec<Value>);

#[derive(Debug, Serialize)]
pub struct Value {
    vname: String,

    #[serde(rename = "type")]
    type_: String,
    value: String,
}

impl Value {
    pub fn new(vname: &str, type_: &str, value: &str) -> Self {
        Self {
            vname: vname.to_string(),
            value: value.to_string(),
            type_: type_.to_string(),
        }
    }
}
