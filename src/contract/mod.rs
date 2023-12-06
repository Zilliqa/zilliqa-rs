pub mod factory;
use std::sync::Arc;

pub use factory::Factory as ContractFactory;
use serde::{Deserialize, Serialize};

use crate::{
    crypto::ZilAddress,
    middlewares::Middleware,
    providers::{CreateTransactionResponse, GetTransactionResponse},
    transaction::TransactionBuilder,
    util::parse_zil,
    Error,
};

#[derive(Debug)]
pub struct BaseContract<T: Middleware> {
    address: ZilAddress,
    client: Arc<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Init(pub Vec<Value>);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Value {
    vname: String,

    #[serde(rename = "type")]
    r#type: String,
    value: String,
}

impl Value {
    pub fn new<T: ToString>(vname: String, r#type: String, value: T) -> Self {
        Self {
            vname,
            value: value.to_string(),
            r#type,
        }
    }

    pub fn new_from_str(vname: &str, r#type: &str, value: &str) -> Self {
        Self {
            vname: vname.to_string(),
            value: value.to_string(),
            r#type: r#type.to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
struct Transition {
    #[serde(rename = "_tag")]
    tag: String,
    params: Vec<Value>,
}

impl<T: Middleware> BaseContract<T> {
    pub fn new(address: ZilAddress, client: Arc<T>) -> Self {
        Self { address, client }
    }

    pub async fn call(&self, transition: &str, args: Vec<Value>) -> Result<GetTransactionResponse, Error> {
        let tx = TransactionBuilder::default()
            .to_address(self.address.clone())
            // TODO: Consider gas price, amount values
            .gas_price(parse_zil("0.002")?)
            .gas_limit(10000u64)
            .data(serde_json::to_string(&Transition {
                tag: transition.to_string(),
                params: args,
            })?)
            .build();

        let response = self
            .client
            .send_transaction_without_confirm::<CreateTransactionResponse>(tx)
            .await?;
        self.client.get_transaction(&response.tran_id).await
    }
}

include!(concat!(env!("OUT_DIR"), "/scilla_contracts.rs"));

// #[derive(Debug)]
// pub struct TestContract<T: Middleware> {
//     pub base: BaseContract<T>,
// }

// impl<T: Middleware> TestContract<T> {
//     pub async fn deploy(client: Arc<T>) -> Result<Self, Error> {
//         let factory = ContractFactory::new(client.clone());
//         let init = Init(vec![
//             Value::new("_scilla_version", "Uint32", "0"),
//             // Value::new("owner", "ByStr20", &wallet.address.to_string()),
//         ]);

//         Ok(Self::new(factory.deploy_from_file(&PathBuf::from("kh"), init, None).await?))
//     }

//     pub fn new(base: BaseContract<T>) -> Self {
//         Self { base }
//     }

//     pub async fn set_hello(&self) -> Result<GetTransactionResponse, Error> {
//         self.base.call("setHello", vec![]).await
//     }

//     pub async fn get_hello(&self) -> Result<GetTransactionResponse, Error> {
//         self.base.call("getHello", vec![]).await
//     }

//     pub async fn throw_error(&self) -> Result<GetTransactionResponse, Error> {
//         self.base.call("throwError", vec![]).await
//     }
// }
