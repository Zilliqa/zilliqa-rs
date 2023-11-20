use std::{cell::RefCell, rc::Rc};

use jsonrpsee::rpc_params;

use crate::{
    account::{wallet::Wallet, Transaction},
    core::HTTPProvider,
    core::RPCMethod,
    crypto::bech32::from_bech32_address,
    util::validation::is_bech32,
};

use super::{
    error::{BlockchainError, ChainResult},
    BalanceResponse, BlockchainInfo, CreateTransactionResponse, DsBlock, ShardingStructure,
};

pub struct Blockchain {
    pub provider: Rc<HTTPProvider>,
    pub signer: Rc<RefCell<Wallet>>,
    version: u32,
}

impl Blockchain {
    pub fn new(provider: Rc<HTTPProvider>, signer: Rc<RefCell<Wallet>>, chain_id: u16) -> Self {
        let msg_version = 1u32;
        let version = (chain_id as u32) << 16 | msg_version;
        Self {
            provider,
            signer,
            version,
        }
    }

    pub async fn get_balance(&self, address: &str) -> ChainResult<BalanceResponse> {
        let address = if is_bech32(address) {
            from_bech32_address(address)?
        } else {
            address.to_string()
        };

        Ok(self
            .provider
            .send(RPCMethod::GetBalance.to_string(), rpc_params![address])
            .await?)
    }

    pub async fn send_transaction(
        &self,
        mut tx: Transaction,
    ) -> ChainResult<CreateTransactionResponse> {
        // Check if version is not set
        if tx.version == u32::default() {
            tx.version = self.version;
        }

        // Sign transaction, this will update pub_key, nonce, and signature.
        let tx = self.signer.borrow().sign_transaction(tx).await?;
        Ok(self
            .provider
            .send(RPCMethod::CreateTransaction.to_string(), rpc_params![tx])
            .await?)
    }

    pub async fn create_transaction(
        &self,
        tx: Transaction,
    ) -> ChainResult<CreateTransactionResponse> {
        Ok(self
            .provider
            .send(RPCMethod::CreateTransaction.to_string(), rpc_params![tx])
            .await?)
    }

    pub async fn get_blockchain_info(&self) -> Result<BlockchainInfo, BlockchainError> {
        Ok(self
            .provider
            .send(RPCMethod::GetBlockchainInfo.to_string(), rpc_params![])
            .await?)
    }

    pub async fn get_sharding_structure(&self) -> ChainResult<ShardingStructure> {
        Ok(self
            .provider
            .send(RPCMethod::GetShardingStructure.to_string(), rpc_params![])
            .await?)
    }

    pub async fn get_ds_block(&self, block_num: usize) -> ChainResult<DsBlock> {
        Ok(self
            .provider
            .send(
                RPCMethod::GetDSBlock.to_string(),
                rpc_params![block_num.to_string()],
            )
            .await?)
    }
}
