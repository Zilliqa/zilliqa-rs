use async_trait::async_trait;
use jsonrpsee::{
    core::{client::ClientT, traits::ToRpcParams},
    http_client::{HttpClient, HttpClientBuilder},
};
use serde::de::DeserializeOwned;
use url::Url;

use super::{error::ProviderError, JsonRpcClient};

#[derive(Debug)]
pub struct Provider {
    client: HttpClient,
}

impl Provider {
    pub fn new(url: impl Into<Url>) -> Result<Self, ProviderError> {
        Ok(Self {
            client: HttpClientBuilder::default().build(url.into())?,
        })
    }
}

#[async_trait]
impl JsonRpcClient for Provider {
    async fn request<T: Send + Sync + ToRpcParams, R: DeserializeOwned>(
        &self,
        method: &str,
        params: T,
    ) -> Result<R, ProviderError> {
        self.client
            .request(&method.to_string(), params)
            .await
            .map_err(ProviderError::JsonRpcError)
    }
}
