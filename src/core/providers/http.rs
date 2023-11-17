use jsonrpsee::{
    core::{client::ClientT, params::ArrayParams},
    http_client::{HttpClient, HttpClientBuilder},
};

use crate::core::{error::CoreError, net::RPCMethod};

pub struct HTTPProvider {
    client: HttpClient,
}

impl HTTPProvider {
    pub fn new(url: &str) -> Result<Self, CoreError> {
        Ok(Self {
            client: HttpClientBuilder::default().build(url)?,
        })
    }

    pub async fn send(
        &self,
        method: RPCMethod,
        params: ArrayParams,
    ) -> Result<String, jsonrpsee::core::Error> {
        self.client.request(&method.to_string(), params).await
    }
}
