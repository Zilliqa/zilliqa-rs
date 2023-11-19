use std::{cell::RefCell, rc::Rc};

use jsonrpsee::rpc_params;

use crate::{
    account::{wallet::Wallet, Transaction},
    core::HTTPProvider,
    core::RPCMethod,
    crypto::bech32::from_bech32_address,
    util::validation::is_bech32,
};

use super::error::BlockchainError;

pub struct Blockchain {
    pub provider: Rc<HTTPProvider>,
    pub signer: Rc<RefCell<Wallet>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct BalanceResponse {
    nonce: u128,
    balance: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct CreateTransactionResponse {
    #[serde(rename = "TranID")]
    tran_id: String,

    #[serde(rename = "Info")]
    info: String,
}

impl Blockchain {
    pub fn new(provider: Rc<HTTPProvider>, signer: Rc<RefCell<Wallet>>) -> Self {
        Self { provider, signer }
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
        tx: Transaction,
    ) -> Result<CreateTransactionResponse, BlockchainError> {
        let tx = self.signer.borrow().sign_transaction(tx)?;
        Ok(self
            .provider
            .send(RPCMethod::CreateTransaction, rpc_params![tx])
            .await?)
    }
}
