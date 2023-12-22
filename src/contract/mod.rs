pub mod factory;
pub mod scilla_value;
pub mod transition_call;
use crate::core::BNum;
use core::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::{ops::Deref, str::FromStr, sync::Arc};

pub use factory::Factory as ContractFactory;
pub use scilla_value::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value as JsonValue;
pub use transition_call::*;

use crate::providers::EventParam;
use crate::signers::Signer;
use crate::{
    crypto::ZilAddress, middlewares::Middleware, providers::GetTransactionResponse, transaction::TransactionParams, Error,
};

#[derive(Debug)]
pub struct BaseContract<T: Middleware> {
    address: ZilAddress,
    client: Arc<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Init(pub Vec<ScillaVariable>);

impl Deref for Init {
    type Target = Vec<ScillaVariable>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Serialize)]
struct Transition {
    #[serde(rename = "_tag")]
    tag: String,
    params: Vec<ScillaVariable>,
}

impl<T: Middleware> BaseContract<T> {
    pub fn new(address: ZilAddress, client: Arc<T>) -> Self {
        Self { address, client }
    }

    pub fn connect<S: Signer>(&self, client: Arc<T>) -> Self {
        Self {
            address: self.address.clone(),
            client,
        }
    }

    /// Call a transition of the contract.
    ///
    /// Arguments:
    ///
    /// * `transition`: A string representing the name of the transition to be called.
    /// * `args`: A vector of ScillaVariable objects, which represents the arguments to be passed to the
    /// transition being called.
    /// * `overridden_params`: An optional parameter that allows you to override the default transaction
    /// parameters. If not provided, it will use the default transaction parameters.
    pub async fn call(
        &self,
        transition: &str,
        args: Vec<ScillaVariable>,
        overridden_params: Option<TransactionParams>,
    ) -> Result<GetTransactionResponse, Error> {
        TransitionCall::new(transition, &self.address, self.client.clone())
            .overridden_params(overridden_params.unwrap_or_default())
            .args(args)
            .call()
            .await
    }

    /// The function `get_field` retrieves a specific field from a smart contract state and parses it into a
    /// specified type.
    ///
    /// Arguments:
    ///
    /// * `field_name`: The `field_name` parameter is a string that represents the name of the field you
    /// want to retrieve from the smart contract state.
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

    /// The function `get_init` retrieves the initialization parameters of a smart contract.
    pub async fn get_init(&self) -> Result<Vec<EventParam>, Error> {
        self.client.get_smart_contract_init(&self.address).await
    }

    /// The function `get_state` retrieves the state of a smart contract asynchronously.
    pub async fn get_state<S: Send + DeserializeOwned>(&self) -> Result<S, Error> {
        self.client.get_smart_contract_state(&self.address).await
    }
}

include!(concat!(env!("OUT_DIR"), "/scilla_contracts.rs"));
