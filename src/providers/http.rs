use async_trait::async_trait;
use jsonrpsee::{
    core::{client::ClientT, traits::ToRpcParams},
    http_client::{HttpClient, HttpClientBuilder},
};
use serde::de::DeserializeOwned;
use url::Url;

use crate::Error;

use super::JsonRpcClient;

#[derive(Debug)]
pub struct Http {
    client: HttpClient,
}

impl Http {
    pub fn new(url: impl Into<Url>) -> Result<Self, Error> {
        Ok(Self {
            client: HttpClientBuilder::default().build(url.into())?,
        })
    }
}

#[async_trait]
impl JsonRpcClient for Http {
    async fn request<T: Send + Sync + ToRpcParams, R: DeserializeOwned>(&self, method: &str, params: T) -> Result<R, Error> {
        self.client.request(method, params).await.map_err(Error::JsonRpcError)
    }
}
