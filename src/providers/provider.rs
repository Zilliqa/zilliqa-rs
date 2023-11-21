use super::net::RPCMethod::*;
use jsonrpsee::rpc_params;
use paste::paste;
use url::Url;

use crate::{account::Transaction, crypto::bech32::from_bech32_address, rpc_method, util::validation::is_bech32};

use super::{
    error::{ProviderError, ProviderResult},
    net::RPCMethod,
    BalanceResponse, BlockList, BlockchainInfo, CreateTransactionResponse, DsBlock, Http, JsonRpcClient, MinerInfo,
    ShardingStructure, TransactionObj, TxBlock, TxList,
};

#[derive(Clone, Debug)]
pub struct Provider<P> {
    inner: P,
}

impl<P: JsonRpcClient> Provider<P> {
    /// Instantiate a new provider with a backend.
    pub fn new(provider: P) -> Self {
        Self { inner: provider }
    }

    pub async fn get_balance(&self, address: &str) -> ProviderResult<BalanceResponse> {
        let address = if is_bech32(address) {
            from_bech32_address(address)?
        } else {
            address.to_string()
        };

        self.inner
            .request(&RPCMethod::GetBalance.to_string(), rpc_params![address])
            .await
    }

    // pub async fn send_transaction(
    //     &self,
    //     mut tx: Transaction,
    // ) -> ChainResult<CreateTransactionResponse> {
    //     // Check if version is not set
    //     if tx.version == u32::default() {
    //         tx.version = self.version;
    //     }

    //     // Sign transaction, this will update pub_key, nonce, and signature.
    //     let tx = self.signer.borrow().sign_transaction(tx).await?;
    //     self.inner
    //         .request(&RPCMethod::CreateTransaction.to_string(), rpc_params![tx])
    //         .await
    // }

    // TODO: zilliqa-js create_transaction is more complex.
    rpc_method!(CreateTransaction, tx, Transaction, CreateTransactionResponse);
    rpc_method!(GetBlockchainInfo, BlockchainInfo);
    rpc_method!(GetShardingStructure, ShardingStructure);
    rpc_method!(GetDsBlock, block_num, &str, DsBlock);
    rpc_method!(GetLatestDsBlock, DsBlock);
    rpc_method!(GetNumDSBlocks, String);
    rpc_method!(GetDSBlockRate, f32);
    rpc_method!(DSBlockListing, max, u32, BlockList);
    rpc_method!(GetTxBlock, block_num, &str, TxBlock);
    rpc_method!(GetLatestTxBlock, TxBlock);
    rpc_method!(GetNumTxBlocks, String);
    rpc_method!(GetTxBlockRate, f32);
    rpc_method!(TxBlockListing, max, u32, BlockList);
    rpc_method!(GetNumTransactions, String);
    rpc_method!(GetTransactionRate, f32);
    rpc_method!(GetCurrentMiniEpoch, String);
    rpc_method!(GetCurrentDSEpoch, String);
    rpc_method!(GetPrevDifficulty, u32);
    rpc_method!(GetPrevDSDifficulty, u32);
    rpc_method!(GetTotalCoinSupply, String);
    rpc_method!(GetMinerInfo, ds_block_number, &str, MinerInfo);
    // TODO: add createBatchTransaction, createTransactionRaw, createTransactionWithoutConfirm, getTransaction, getTransactionStatus
    rpc_method!(GetRecentTransactions, TxList);
    rpc_method!(GetTransactionsForTxBlock, tx_block, &str, Vec<Vec<String>>);
    rpc_method!(GetTransactionsForTxBlockEx, tx_block, usize, Vec<Vec<String>>);
    rpc_method!(GetTxnBodiesForTxBlock, tx_block, usize, TransactionObj);
    rpc_method!(GetTxnBodiesForTxBlockEx, tx_block, usize, TransactionObj);
    rpc_method!(GetNumTxnsTxEpoch, epoch, &str, String);
    rpc_method!(GetNumTxnsDSEpoch, epoch, &str, String);
    rpc_method!(GetMinimumGasPrice, String);
    // TODO: add   getSmartContractCode, getSmartContractInit, getSmartContractState, getSmartContractSubState, getSmartContractSubStateBatch
    rpc_method!(GetContractAddressFromTransactionID, tx_hash, String, String);
    // TODO: add getStateProof
}

impl TryFrom<&str> for Provider<Http> {
    type Error = ProviderError;

    fn try_from(src: &str) -> Result<Self, Self::Error> {
        Ok(Provider::new(Http::new(Url::parse(src)?)?))
    }
}
