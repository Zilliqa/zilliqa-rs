use crate::core::{
    types::RPCMethod::{self, *},
    types::*,
    TxHash,
};
use async_trait::async_trait;
use jsonrpsee::{core::params::ArrayParams, rpc_params};
use serde::de::DeserializeOwned;
use url::Url;

use crate::{
    core::ZilAddress,
    crypto::Signature,
    middlewares::{signer::SignerMiddleware, Middleware},
    signers::Signer,
    Error,
};

use super::{Http, JsonRpcClient};

/// # Example
/// ## From a URL
/// ```
/// use zilliqa_rs::providers::{Http, Provider};
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let provider = Provider::<Http>::try_from("http://127.0.0.1:5555")?;
///     Ok(())
/// }
/// ```
///
/// ## With chain ID
/// ```
/// use zilliqa_rs::providers::{Http, Provider};
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let provider = Provider::<Http>::try_from("http://127.0.0.1:5555").unwrap().with_chain_id(1);
///     Ok(())
/// }
/// ```
/// ## With a signer
/// If a provider has a designated signer, all transactions requiring signing will be signed using the designated signer before being sent to the endpoint.
///
/// ```
/// use zilliqa_rs::providers::{Http, Provider};
/// use zilliqa_rs::signers::LocalWallet;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let wallet = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7".parse::<LocalWallet>()?;
///     let provider = Provider::<Http>::try_from("http://127.0.0.1").unwrap().with_signer(wallet);
///     Ok(())
/// }
/// ```
/// ## Call RPC methods
/// ```
/// use zilliqa_rs::providers::{Http, Provider};
/// use zilliqa_rs::middlewares::Middleware;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let provider = Provider::<Http>::try_from("http://127.0.0.1:5555")?.with_chain_id(222);
///     let balance = provider.get_balance("0x381f4008505e940ad7681ec3468a719060caf796").await;
///     Ok(())
/// }
/// ```
#[derive(Clone, Debug)]
pub struct Provider<P> {
    inner: P,
    chain_id: u16,
}

impl<P: JsonRpcClient> Provider<P> {
    /// Instantiate a new provider with a backend.
    pub fn new(provider: P, chain_id: u16) -> Self {
        Self {
            inner: provider,
            chain_id,
        }
    }

    pub fn with_signer<S: Signer>(self, signer: S) -> SignerMiddleware<Self, S> {
        SignerMiddleware::new(self, signer)
    }

    pub fn with_chain_id(mut self, chain_id: u16) -> Self {
        self.chain_id = chain_id;
        self
    }

    pub async fn send_request<T: Send + DeserializeOwned>(&self, rpc: RPCMethod, params: ArrayParams) -> Result<T, Error> {
        self.inner.request(&rpc.to_string(), params).await
    }
}

impl TryFrom<&str> for Provider<Http> {
    type Error = Error;

    fn try_from(src: &str) -> Result<Self, Self::Error> {
        Ok(Provider::new(Http::new(Url::parse(src)?)?, u16::default()))
    }
}

impl TryFrom<Url> for Provider<Http> {
    type Error = Error;

    fn try_from(url: Url) -> Result<Self, Self::Error> {
        Ok(Provider::new(Http::new(url)?, u16::default()))
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

    fn chainid(&self) -> u16 {
        self.chain_id
    }

    fn sign_transaction(&self, _tx: &CreateTransactionRequest) -> Result<Signature, Error> {
        Err(Error::NoSignerSpecified)
    }

    async fn send_transaction_without_confirm<T: Send + DeserializeOwned>(
        &self,
        mut _tx: CreateTransactionRequest,
    ) -> Result<T, Error> {
        Err(Error::NoSignerSpecified)
    }

    async fn create_transaction<T: DeserializeOwned + Send>(&self, tx: CreateTransactionRequest) -> Result<T, Error> {
        if !tx.version.valid() {
            return Err(Error::InvalidVersionIsSetForTransaction(tx.version));
        }

        Ok(self.send_request(CreateTransaction, rpc_params![tx]).await?)
    }

    async fn get_transaction(&self, tx_hash: &TxHash) -> Result<GetTransactionResponse, Error> {
        Ok(self.send_request(GetTransaction, rpc_params![tx_hash.to_string()]).await?)
    }

    async fn get_transaction_status(&self, tx_hash: &TxHash) -> Result<TransactionStatus, Error> {
        Ok(self
            .send_request(GetTransactionStatus, rpc_params![tx_hash.to_string()])
            .await?)
    }

    async fn get_balance(&self, address: &str) -> Result<BalanceResponse, Error> {
        Ok(self.send_request(GetBalance, rpc_params![address]).await?)
    }

    async fn get_ds_block(&self, lock_num: &str) -> Result<DsBlock, Error> {
        Ok(self.send_request(GetDsBlock, rpc_params![lock_num]).await?)
    }

    async fn ds_block_listing(&self, max: u32) -> Result<BlockList, Error> {
        Ok(self.send_request(DsBlockListing, rpc_params![max]).await?)
    }

    async fn get_tx_block(&self, block_num: &str) -> Result<TxBlock, Error> {
        Ok(self.send_request(GetTxBlock, rpc_params![block_num]).await?)
    }

    async fn tx_block_listing(&self, max: u32) -> Result<BlockList, Error> {
        Ok(self.send_request(GetTxBlock, rpc_params![max]).await?)
    }

    async fn get_miner_info(&self, ds_block_number: &str) -> Result<MinerInfo, Error> {
        Ok(self.send_request(GetMinerInfo, rpc_params![ds_block_number]).await?)
    }

    async fn get_blockchain_info(&self) -> Result<BlockchainInfo, Error> {
        Ok(self.send_request(GetBlockchainInfo, rpc_params![]).await?)
    }

    async fn get_sharding_structure(&self) -> Result<ShardingStructure, Error> {
        Ok(self.send_request(GetShardingStructure, rpc_params![]).await?)
    }

    async fn get_latest_ds_block(&self) -> Result<DsBlock, Error> {
        Ok(self.send_request(GetLatestDsBlock, rpc_params![]).await?)
    }

    async fn get_num_ds_blocks(&self) -> Result<String, Error> {
        Ok(self.send_request(GetNumDsBlocks, rpc_params![]).await?)
    }

    async fn get_ds_block_rate(&self) -> Result<f32, Error> {
        Ok(self.send_request(GetDsBlockRate, rpc_params![]).await?)
    }

    async fn get_latest_tx_block(&self) -> Result<TxBlock, Error> {
        Ok(self.send_request(GetLatestTxBlock, rpc_params![]).await?)
    }

    async fn get_num_tx_blocks(&self) -> Result<String, Error> {
        Ok(self.send_request(GetNumTxBlocks, rpc_params![]).await?)
    }

    async fn get_tx_block_rate(&self) -> Result<f32, Error> {
        Ok(self.send_request(GetTxBlockRate, rpc_params![]).await?)
    }

    async fn get_num_transactions(&self) -> Result<String, Error> {
        Ok(self.send_request(GetNumTransactions, rpc_params![]).await?)
    }

    async fn get_transaction_rate(&self) -> Result<f32, Error> {
        Ok(self.send_request(GetTransactionRate, rpc_params![]).await?)
    }

    async fn get_current_mini_epoch(&self) -> Result<String, Error> {
        Ok(self.send_request(GetCurrentMiniEpoch, rpc_params![]).await?)
    }

    async fn get_current_ds_epoch(&self) -> Result<String, Error> {
        Ok(self.send_request(GetCurrentDsEpoch, rpc_params![]).await?)
    }

    async fn get_prev_difficulty(&self) -> Result<u32, Error> {
        Ok(self.send_request(GetPrevDifficulty, rpc_params![]).await?)
    }

    async fn get_prev_ds_difficulty(&self) -> Result<u32, Error> {
        Ok(self.send_request(GetPrevDsDifficulty, rpc_params![]).await?)
    }

    async fn get_total_coin_supply(&self) -> Result<String, Error> {
        Ok(self.send_request(GetTotalCoinSupply, rpc_params![]).await?)
    }

    async fn get_recent_transactions(&self) -> Result<TxList, Error> {
        Ok(self.send_request(GetRecentTransactions, rpc_params![]).await?)
    }

    async fn get_transactions_for_tx_block(&self, tx_block: &str) -> Result<Vec<Vec<String>>, Error> {
        Ok(self.send_request(GetTransactionsForTxBlock, rpc_params![tx_block]).await?)
    }

    async fn get_txn_bodies_for_tx_block_ex(&self, tx_block: &str, page_num: &str) -> Result<TxnBodiesForTxBlockEx, Error> {
        Ok(self
            .send_request(GetTxnBodiesForTxBlockEx, rpc_params![tx_block, page_num])
            .await?)
    }

    async fn get_txn_bodies_for_tx_block(&self, tx_block: &str) -> Result<Vec<GetTransactionResponse>, Error> {
        Ok(self.send_request(GetTxnBodiesForTxBlock, rpc_params![tx_block]).await?)
    }

    async fn get_transactions_for_tx_block_ex(&self, tx_block: &str, page_num: &str) -> Result<TransactionsForTxBlockEx, Error> {
        Ok(self
            .send_request(GetTransactionsForTxBlockEx, rpc_params![tx_block, page_num])
            .await?)
    }

    async fn get_num_txns_tx_epoch(&self, epoch: &str) -> Result<String, Error> {
        Ok(self.send_request(GetNumTxnsTxEpoch, rpc_params![epoch]).await?)
    }

    async fn get_num_txns_ds_epoch(&self, epoch: &str) -> Result<String, Error> {
        Ok(self.send_request(GetNumTxnsDsEpoch, rpc_params![epoch]).await?)
    }

    async fn get_minimum_gas_price(&self) -> Result<String, Error> {
        Ok(self.send_request(GetMinimumGasPrice, rpc_params![]).await?)
    }

    async fn get_contract_address_from_transaction_id(&self, tx_hash: &TxHash) -> Result<String, Error> {
        Ok(self
            .send_request(GetContractAddressFromTransactionId, rpc_params![tx_hash.to_string()])
            .await?)
    }

    async fn get_smart_contracts(&self, owner: &ZilAddress) -> Result<SmartContracts, Error> {
        Ok(self.send_request(GetSmartContracts, rpc_params![owner]).await?)
    }

    async fn get_smart_contract_code(&self, contract_address: &ZilAddress) -> Result<SmartContractCode, Error> {
        Ok(self.send_request(GetSmartContractCode, rpc_params![contract_address]).await?)
    }

    async fn get_smart_contract_init(&self, contract_address: &ZilAddress) -> Result<Vec<EventParam>, Error> {
        Ok(self.send_request(GetSmartContractInit, rpc_params![contract_address]).await?)
    }

    async fn get_smart_contract_state<T: Send + DeserializeOwned>(&self, contract_address: &ZilAddress) -> Result<T, Error> {
        Ok(self
            .send_request(GetSmartContractState, rpc_params![contract_address])
            .await?)
    }

    async fn get_smart_contract_sub_state(
        &self,
        contract_address: &ZilAddress,
        variable_name: &str,
        indices: &[&str],
    ) -> Result<serde_json::Value, Error> {
        Ok(self
            .send_request(
                GetSmartContractSubState,
                rpc_params![contract_address, variable_name, indices],
            )
            .await?)
    }

    async fn get_state_proof(
        &self,
        contract_address: &ZilAddress,
        hash: &str,
        tx_block: &str,
    ) -> Result<serde_json::Value, Error> {
        Ok(self
            .send_request(GetStateProof, rpc_params![contract_address, hash, tx_block])
            .await?)
    }
}
