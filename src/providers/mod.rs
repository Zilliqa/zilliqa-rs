//! Clients for interacting with zilliqa network.

pub mod http;
pub mod provider;

pub use http::Http;
pub use provider::Provider;

use crate::Error;
use async_trait::async_trait;
use jsonrpsee::core::traits::ToRpcParams;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

/// JSON-RPC client trait
///
/// If a client wants to be used as a JSON-RPC client, it must implement this trait.
#[async_trait]
pub trait JsonRpcClient: Debug + Send + Sync {
    async fn request<T, R>(&self, method: &str, params: T) -> Result<R, Error>
    where
        T: Debug + Send + Sync + ToRpcParams,
        R: DeserializeOwned + Send;
}
