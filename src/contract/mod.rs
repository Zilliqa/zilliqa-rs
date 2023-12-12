pub mod factory;
pub mod transition_call;
use core::cell::{RefCell, RefMut};
use std::{str::FromStr, sync::Arc};

pub use factory::Factory as ContractFactory;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value as JsonValue;
pub use transition_call::*;

use crate::{
    crypto::ZilAddress, middlewares::Middleware, providers::EventParam, providers::GetTransactionResponse,
    transaction::TransactionParams, Error,
};

pub type Value = EventParam;

#[derive(Debug)]
pub struct BaseContract<T: Middleware> {
    address: ZilAddress,
    client: Arc<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Init(pub Vec<Value>);

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

    pub async fn call(
        &self,
        transition: &str,
        args: Vec<Value>,
        overridden_params: Option<TransactionParams>,
    ) -> Result<GetTransactionResponse, Error> {
        TransitionCall::new(transition, &self.address, self.client.clone())
            .overridden_params(overridden_params.unwrap_or_default())
            .args(args)
            .call()
            .await
    }

    pub async fn get_field<F: FromStr>(&self, field_name: &str) -> Result<F, Error> {
        let state = self.client.get_smart_contract_state(&self.address).await?;
        if let JsonValue::Object(object) = state {
            if let Some(value) = object.get(field_name) {
                return value
                    .to_string()
                    .parse::<F>()
                    .map_err(|_| Error::FailedToParseContractField(field_name.to_string()));
            }
        }
        Err(Error::NoSuchFieldInContractState(field_name.to_string()))
    }

    pub async fn get_state<S: Send + DeserializeOwned>(&self) -> Result<S, Error> {
        self.client.get_smart_contract_state(&self.address).await
    }
}

include!(concat!(env!("OUT_DIR"), "/scilla_contracts.rs"));
