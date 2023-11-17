use jsonrpsee::rpc_params;

use crate::{
    core::HTTPProvider, core::RPCMethod, crypto::bech32::from_bech32_address,
    util::validation::is_bech32,
};

use super::error::BlockchainError;

pub struct Blockchain {
    pub provider: HTTPProvider,
}

impl Blockchain {
    pub fn new(provider: HTTPProvider) -> Self {
        Self { provider }
    }

    pub async fn get_balance(&self, address: &str) -> Result<String, BlockchainError> {
        let address = if is_bech32(address) {
            from_bech32_address(&address)?
        } else {
            address.to_string()
        };

        Ok(self
            .provider
            .send(RPCMethod::GetBalance, rpc_params![address])
            .await?)
    }
}
