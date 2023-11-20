use jsonrpsee::{
    core::{client::ClientT, params::ArrayParams},
    http_client::{HttpClient, HttpClientBuilder},
};
use serde::de::DeserializeOwned;

use crate::core::error::CoreError;

pub struct HTTPProvider {
    client: HttpClient,
}

impl HTTPProvider {
    pub fn new(url: &str) -> Result<Self, CoreError> {
        Ok(Self {
            client: HttpClientBuilder::default().build(url)?,
        })
    }

    pub async fn send<R>(
        &self,
        method: String,
        params: ArrayParams,
    ) -> Result<R, jsonrpsee::core::Error>
    where
        R: DeserializeOwned,
    {
        self.client.request(&method.to_string(), params).await
    }
}
