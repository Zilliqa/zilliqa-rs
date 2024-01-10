pub mod http;
mod macros;
pub mod provider;

pub use http::Http;
pub use provider::Provider;

use crate::Error;
use async_trait::async_trait;
use jsonrpsee::core::traits::ToRpcParams;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

#[async_trait]
pub trait JsonRpcClient: Debug + Send + Sync {
    async fn request<T, R>(&self, method: &str, params: T) -> Result<R, Error>
    where
        T: Debug + Send + Sync + ToRpcParams,
        R: DeserializeOwned + Send;
}
