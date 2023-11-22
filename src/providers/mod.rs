pub mod error;
pub mod http;
mod macros;
pub mod net;
pub mod provider;
pub mod types;

pub use error::ProviderError;
pub use http::Provider as Http;
pub use provider::Provider;
pub use types::*;

use async_trait::async_trait;
use jsonrpsee::core::traits::ToRpcParams;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

#[async_trait]
pub trait JsonRpcClient: Debug + Send + Sync {
    async fn request<T, R>(&self, method: &str, params: T) -> Result<R, ProviderError>
    where
        T: Debug + Send + Sync + ToRpcParams,
        R: DeserializeOwned + Send;
}
