use std::{cell::RefCell, rc::Rc};

use jsonrpsee::rpc_params;

use crate::{
    account::{wallet::Wallet, Transaction},
    core::HTTPProvider,
    core::RPCMethod,
    crypto::bech32::from_bech32_address,
    util::validation::is_bech32,
};

use super::{error::BlockchainError, BalanceResponse, CreateTransactionResponse};

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

    pub async fn get_balance(&self, address: &str) -> Result<BalanceResponse, BlockchainError> {
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

    pub async fn send_transaction(
        &self,
        mut tx: Transaction,
    ) -> Result<CreateTransactionResponse, BlockchainError> {
        // Check if version is not set
        if tx.version == u32::default() {
            tx.version = self.version;
        }

        // Sign transaction, this will update pub_key, nonce, and signature.
        let tx = self.signer.borrow().sign_transaction(tx).await?;
        Ok(self
            .provider
            .send(RPCMethod::CreateTransaction, rpc_params![tx])
            .await?)
    }

    pub async fn create_transaction(
        &self,
        tx: Transaction,
    ) -> Result<CreateTransactionResponse, BlockchainError> {
        Ok(self
            .provider
            .send(RPCMethod::CreateTransaction, rpc_params![tx])
            .await?)
    }
}
