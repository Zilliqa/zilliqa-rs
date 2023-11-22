use super::{
    net::RPCMethod::{self, *},
    TransactionsForTxBlockEx, TxnBodiesForTxBlockEx,
};
use async_trait::async_trait;
use jsonrpsee::{core::params::ArrayParams, rpc_params};
use serde::de::DeserializeOwned;
use url::Url;

use crate::{
    account::Transaction,
    middlewares::{signer::SignerMiddleware, Middleware, MiddlewareError, MiddlewareResult},
    signers::LocalWallet,
};

use super::{
    error::{ProviderError, ProviderResult},
    BalanceResponse, BlockList, BlockchainInfo, CreateTransactionResponse, DsBlock, Http, JsonRpcClient, MinerInfo,
    ShardingStructure, TransactionObj, TxBlock, TxList,
};

#[derive(Clone, Debug)]
pub struct Provider<P> {
    inner: P,
    chain_id: u64,
}

impl<P: JsonRpcClient> Provider<P> {
    /// Instantiate a new provider with a backend.
    pub fn new(provider: P, chain_id: u64) -> Self {
        Self {
            inner: provider,
            chain_id,
        }
    }

    pub fn with_signer(self, signer: LocalWallet) -> SignerMiddleware<Self> {
        SignerMiddleware::new(self, signer)
    }

    pub fn with_chain_id(mut self, chain_id: u64) -> Self {
        self.chain_id = chain_id;
        self
    }

    pub async fn send_request<T: Send + DeserializeOwned>(&self, rpc: RPCMethod, params: ArrayParams) -> ProviderResult<T> {
        self.inner.request(&rpc.to_string(), params).await
    }

    // TODO: zilliqa-js create_transaction is more complex.
    // TODO: add createBatchTransaction, createTransactionRaw, createTransactionWithoutConfirm, getTransaction, getTransactionStatus
    // TODO: add GetContractAddressFromTransactionID, getSmartContractCode, getSmartContractInit, getSmartContractState, getSmartContractSubState, getSmartContractSubStateBatch
    // TODO: add getStateProof
}

impl TryFrom<&str> for Provider<Http> {
    type Error = ProviderError;

    fn try_from(src: &str) -> Result<Self, Self::Error> {
        Ok(Provider::new(Http::new(Url::parse(src)?)?, u64::default()))
    }
}

#[async_trait]
impl<P: JsonRpcClient> Middleware for Provider<P> {
    type Provider = P;

    type Inner = Self;

    fn inner(&self) -> &Self::Inner {
        unreachable!("There is no inner provider here")
    }

    fn provider(&self) -> &Provider<Self::Provider> {
        self
    }

    fn get_chainid(&self) -> u64 {
        self.chain_id
    }

    fn sign_transaction(&self, _tx: &Transaction) -> MiddlewareResult<k256::ecdsa::Signature> {
        Err(MiddlewareError::ProviderError(ProviderError::NoSignerSpecified))
    }

    async fn send_transaction(&self, mut _tx: Transaction) -> MiddlewareResult<CreateTransactionResponse> {
        Err(MiddlewareError::NoSignerCaughtTheSendRequest)
    }

    async fn create_transaction(&self, tx: Transaction) -> MiddlewareResult<CreateTransactionResponse> {
        Ok(self.send_request(CreateTransaction, rpc_params![tx]).await?)
    }

    async fn get_balance(&self, address: &str) -> MiddlewareResult<BalanceResponse> {
        Ok(self.send_request(GetBalance, rpc_params![address]).await?)
    }

    async fn get_ds_block(&self, lock_num: &str) -> MiddlewareResult<DsBlock> {
        Ok(self.send_request(GetDsBlock, rpc_params![lock_num]).await?)
    }

    async fn ds_block_listing(&self, max: u32) -> MiddlewareResult<BlockList> {
        Ok(self.send_request(DsBlockListing, rpc_params![max]).await?)
    }

    async fn get_tx_block(&self, block_num: &str) -> MiddlewareResult<TxBlock> {
        Ok(self.send_request(GetTxBlock, rpc_params![block_num]).await?)
    }

    async fn tx_block_listing(&self, max: u32) -> MiddlewareResult<BlockList> {
        Ok(self.send_request(GetTxBlock, rpc_params![max]).await?)
    }

    async fn get_miner_info(&self, ds_block_number: &str) -> MiddlewareResult<MinerInfo> {
        Ok(self.send_request(GetMinerInfo, rpc_params![ds_block_number]).await?)
    }

    async fn get_blockchain_info(&self) -> MiddlewareResult<BlockchainInfo> {
        Ok(self.send_request(GetBlockchainInfo, rpc_params![]).await?)
    }

    async fn get_sharding_structure(&self) -> MiddlewareResult<ShardingStructure> {
        Ok(self.send_request(GetShardingStructure, rpc_params![]).await?)
    }

    async fn get_latest_ds_block(&self) -> MiddlewareResult<DsBlock> {
        Ok(self.send_request(GetLatestDsBlock, rpc_params![]).await?)
    }

    async fn get_num_ds_blocks(&self) -> MiddlewareResult<String> {
        Ok(self.send_request(GetNumDsBlocks, rpc_params![]).await?)
    }

    async fn get_ds_block_rate(&self) -> MiddlewareResult<f32> {
        Ok(self.send_request(GetDsBlockRate, rpc_params![]).await?)
    }

    async fn get_latest_tx_block(&self) -> MiddlewareResult<TxBlock> {
        Ok(self.send_request(GetLatestTxBlock, rpc_params![]).await?)
    }

    async fn get_num_tx_blocks(&self) -> MiddlewareResult<String> {
        Ok(self.send_request(GetNumTxBlocks, rpc_params![]).await?)
    }

    async fn get_tx_block_rate(&self) -> MiddlewareResult<f32> {
        Ok(self.send_request(GetTxBlockRate, rpc_params![]).await?)
    }

    async fn get_num_transactions(&self) -> MiddlewareResult<String> {
        Ok(self.send_request(GetNumTransactions, rpc_params![]).await?)
    }

    async fn get_transaction_rate(&self) -> MiddlewareResult<f32> {
        Ok(self.send_request(GetTransactionRate, rpc_params![]).await?)
    }

    async fn get_current_mini_epoch(&self) -> MiddlewareResult<String> {
        Ok(self.send_request(GetCurrentMiniEpoch, rpc_params![]).await?)
    }

    async fn get_current_ds_epoch(&self) -> MiddlewareResult<String> {
        Ok(self.send_request(GetCurrentDsEpoch, rpc_params![]).await?)
    }

    async fn get_prev_difficulty(&self) -> MiddlewareResult<u32> {
        Ok(self.send_request(GetPrevDifficulty, rpc_params![]).await?)
    }

    async fn get_prev_ds_difficulty(&self) -> MiddlewareResult<u32> {
        Ok(self.send_request(GetPrevDsDifficulty, rpc_params![]).await?)
    }

    async fn get_total_coin_supply(&self) -> MiddlewareResult<String> {
        Ok(self.send_request(GetTotalCoinSupply, rpc_params![]).await?)
    }

    async fn get_recent_transactions(&self) -> MiddlewareResult<TxList> {
        Ok(self.send_request(GetRecentTransactions, rpc_params![]).await?)
    }

    async fn get_transactions_for_tx_block(&self, tx_block: &str) -> MiddlewareResult<Vec<Vec<String>>> {
        Ok(self.send_request(GetTransactionsForTxBlock, rpc_params![tx_block]).await?)
    }

    async fn get_txn_bodies_for_tx_block_ex(&self, tx_block: &str, page_num: &str) -> MiddlewareResult<TxnBodiesForTxBlockEx> {
        Ok(self
            .send_request(GetTxnBodiesForTxBlockEx, rpc_params![tx_block, page_num])
            .await?)
    }

    async fn get_txn_bodies_for_tx_block(&self, tx_block: &str) -> MiddlewareResult<Vec<TransactionObj>> {
        Ok(self.send_request(GetTxnBodiesForTxBlock, rpc_params![tx_block]).await?)
    }

    async fn get_transactions_for_tx_block_ex(
        &self,
        tx_block: &str,
        page_num: &str,
    ) -> MiddlewareResult<TransactionsForTxBlockEx> {
        Ok(self
            .send_request(GetTransactionsForTxBlockEx, rpc_params![tx_block, page_num])
            .await?)
    }

    async fn get_num_txns_tx_epoch(&self, epoch: &str) -> MiddlewareResult<String> {
        Ok(self.send_request(GetNumTxnsTxEpoch, rpc_params![epoch]).await?)
    }

    async fn get_num_txns_ds_epoch(&self, epoch: &str) -> MiddlewareResult<String> {
        Ok(self.send_request(GetNumTxnsDsEpoch, rpc_params![epoch]).await?)
    }

    async fn get_minimum_gas_price(&self) -> MiddlewareResult<String> {
        Ok(self.send_request(GetMinimumGasPrice, rpc_params![]).await?)
    }
}
