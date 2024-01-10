//!
//! # Working with Contracts
//! One of the coolest features of zilliqa-rs is generating rust code for your scilla contracts during build time. It means if your contract has a transition like `transfer`, you can call it the same as a normal rust function. If it has a parameter of an address, you must pass an address to this function. And this means all of the beauties of type-checking of rust come to working with scilla contracts.
//!
//! ## Generating rust code from scilla contracts
//! We want to deploy a simple contract named `HelloWorld` and call its `setHello` transition. First, we need to create a folder next to `src`. Let's call it `contracts`. Then we move [HelloWorld.scilla](./tests/contracts/HelloWorld.scilla) to this folder. To let zilliqa-rs scilla-to-rust code generation know about the contracts path, we need to export `CONTRACTS_PATH` environment variable. The simplest way is to create `.cargo/config.toml` file and change it like:
//!
//! ```toml
//! [env]
//! CONTRACTS_PATH = {value = "contracts", relative = true}
//! ```
//! setting `relative` to `true` is crucial. Otherwise, your scilla contracts won't be transpiled to rust. Now, if you build the project using `cargo build`, your HelloWorld.scilla gets converted to rust under the hood.
//!
//! The generated code is something like this:
//!
//! ```rust
//! impl<T: Middleware> HelloWorld<T> {
//!     pub async fn deploy(client: Arc<T> , owner: ZilAddress) -> Result<Self, Error> {
//!     }
//!
//!     pub fn address(&self) -> &ZilAddress  {
//!     }
//!     
//!     pub fn set_hello(&self , msg: String) -> RefMut<'_, transition_call::TransitionCall<T>> {
//!     }
//!
//!     pub fn get_hello(&self ) -> RefMut<'_, transition_call::TransitionCall<T>> {
//!     }
//!
//!     pub async fn welcome_msg(&self) -> Result<String, Error> {
//!     }
//!
//!     pub async fn owner(&self) -> Result<ZilAddress, Error> {
//!     }
//! }
//! ```
//! * The `deploy` deploys the contract to the network. Because HelloWorld.scilla contract accepts an address, `owner`, as a deployment parameter, the `deploy` function needs that too. It means you can't deploy it without providing a valid address.
//! * The `address` function returns the address of the deployed contract.
//! * `set_hello` corresponds to `setHello` transition in the contract. Again, because the transition accepts a string parameter, the `set_hello` function does too.
//! * `get_hello` corresponds to the `getHello` transition.
//! * The contract has a field named, `welcome_msg`, to get the value of this field, the `welcome_msg` function should be called.
//! * The contract has an immutable state named, `owner` and we passed the value during deployment. To get the value of the owner, we need to call `owner`
//!
//! # Examples
//!
//! ```
//! use std::sync::Arc;
//!
//! use zilliqa_rs::{
//!     contract,
//!     providers::{Http, Provider},
//!     signers::LocalWallet,
//! };
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     const END_POINT: &str = "http://localhost:5555";
//!
//!     let wallet = "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba".parse::<LocalWallet>()?;
//!
//!     let provider = Provider::<Http>::try_from(END_POINT)?
//!         .with_chain_id(222)
//!         .with_signer(wallet.clone());
//!
//!     let contract = contract::Timestamp::deploy(Arc::new(provider)).await?;
//!
//!     Ok(())
//! }
//!
//! ```
//! If the contract needs some initial parameters, You must pass them to `deploy` function, otherwise you won't be able to compile the code.
//!
//! Instead of using rust binding, it's possible to use `deploy_from_file` or `deploy_str` functions from `ContractFactory` to deploy a contract manually.
//!
//! ### Calling a transition
//!
//! The [HelloWorld](./tests/contracts/HelloWorld.scilla) contract has a `setHello` transition. It can be called in rust like:
//! ```
//! use std::sync::Arc;
//!
//! use zilliqa_rs::{
//!     contract,
//!     core::BNum,
//!     providers::{Http, Provider},
//!     signers::LocalWallet,
//! };
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     const END_POINT: &str = "http://localhost:5555";
//!
//!     let wallet = "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba".parse::<LocalWallet>()?;
//!
//!     let provider = Provider::<Http>::try_from(END_POINT)?
//!         .with_chain_id(222)
//!         .with_signer(wallet.clone());
//!
//!     let contract = contract::HelloWorld::deploy(Arc::new(provider), wallet.address.clone()).await?;
//!     contract.set_hello("Salaam".to_string()).call().await?;
//!
//!     Ok(())
//! }
//! ```
//! If a transition needs some parameters, like here, You must pass them too, otherwise you won't be able to compile the code.
//!
//! ### Calling a transaction with custom parameters for nonce, amount, etc.
//! It's possible to override default transaction parameters such as nonce and amount.
//! ```
//! use zilliqa_rs::{contract, middlewares::Middleware, util::parse_zil, signers::LocalWallet, providers::{Http, Provider}};
//! use std::sync::Arc;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     const END_POINT: &str = "http://localhost:5555";
//!
//!     let wallet = "e53d1c3edaffc7a7bab5418eb836cf75819a82872b4a1a0f1c7fcf5c3e020b89".parse::<LocalWallet>()?;
//!
//!     let provider = Arc::new(Provider::<Http>::try_from(END_POINT)?
//!         .with_chain_id(222)
//!         .with_signer(wallet.clone()));
//!
//!     let contract = contract::SendZil::deploy(provider.clone()).await?;
//!     // Override the amount before sending the transaction.
//!     contract.accept_zil().amount(parse_zil("0.5")?).call().await?;
//!     assert_eq!(provider.get_balance(contract.address()).await?.balance, parse_zil("0.5")?);
//!     Ok(())
//! }
//! ```
//!
//! It's possible to call a transition without using rust binding. Take a look at `call_a_param_less_transition` and `call_transition_with_single_string_param` tests.
//!
//! ### Getting the contract's state
//! Suppose we have a contract like this:
//! ```scilla
//! contract HelloWorld
//! (owner: ByStr20)
//!
//! field welcome_msg : String = "Hello world!"
//! ```
//! You can get the latest state of the contract by calling `welcome_msg` function:
//! ```rust
//! use std::sync::Arc;
//!
//! use zilliqa_rs::{
//!     contract,
//!     providers::{Http, Provider},
//!     signers::LocalWallet,
//! };
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     const END_POINT: &str = "http://localhost:5555";
//!
//!     let wallet = "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba".parse::<LocalWallet>()?;
//!
//!     let provider = Provider::<Http>::try_from(END_POINT)?
//!         .with_chain_id(222)
//!         .with_signer(wallet.clone());
//!
//!     let contract = contract::HelloWorld::deploy(Arc::new(provider), wallet.address.clone()).await?;
//!
//!     let hello = contract.welcome_msg().await?;
//!     assert_eq!(hello, "Hello world!".to_string());
//!
//!     contract.set_hello("Salaam".to_string()).call().await?;
//!     let hello = contract.welcome_msg().await?;
//!     assert_eq!(hello, "Salaam".to_string());
//!     Ok(())
//! }

pub mod factory;
pub mod scilla_value;
pub mod transition_call;
use std::{ops::Deref, str::FromStr, sync::Arc};

pub use factory::Factory as ContractFactory;
pub use scilla_value::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value as JsonValue;
pub use transition_call::*;

use crate::core::{EventParam, GetTransactionResponse};
use crate::signers::Signer;
use crate::{crypto::ZilAddress, middlewares::Middleware, transaction::TransactionParams, Error};

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
