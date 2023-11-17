use jsonrpsee::rpc_params;

use crate::core::HTTPProvider;

pub struct Blockchain {
    pub provider: HTTPProvider,
}

impl Blockchain {
    pub fn new(provider: HTTPProvider) -> Self {
        Self { provider }
    }

    pub async fn get_balance(&self, address: &str) -> Result<String, jsonrpsee::core::Error> {
        self.provider.send("GetBalance", rpc_params![address]).await
    }
}
