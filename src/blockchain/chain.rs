use std::{cell::RefCell, rc::Rc};

use jsonrpsee::rpc_params;

use crate::{
    account::{wallet::Wallet, Transaction},
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

macro_rules! rpc_method {
    ($func:ident, $rpc:expr, $ret:ty) => {
        pub async fn $func(&self) -> ChainResult<$ret> {
            Ok(self.provider.send($rpc, rpc_params![]).await?)
        }
    };
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

    rpc_method!(
        get_latest_ds_block,
        RPCMethod::GetLatestDsBlock.to_string(),
        DsBlock
    );
    // pub async fn get_latest_ds_block(&self) -> ChainResult<DsBlock> {
    //     Ok(self
    //         .provider
    //         .send(RPCMethod::GetLatestDSBlock.to_string(), rpc_params![])
    //         .await?)
    // }
}
