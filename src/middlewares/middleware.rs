use crate::{
    core::{types::*, TxHash, ZilAddress},
    crypto::Signature,
    providers::{JsonRpcClient, Provider},
    transaction::Transaction,
    Error,
};
use async_trait::async_trait;
use serde::de::DeserializeOwned;

#[async_trait]
pub trait Middleware: Sync + Send + std::fmt::Debug {
    /// The JSON-RPC client type at the bottom of the stack
    type Provider: JsonRpcClient;
    /// The next-lower middleware in the middleware stack
    type Inner: Middleware<Provider = Self::Provider>;

    /// Get a reference to the next-lower middleware in the middleware stack
    fn inner(&self) -> &Self::Inner;

    /// The HTTP or Websocket provider.
    fn provider(&self) -> &Provider<Self::Provider> {
        self.inner().provider()
    }

    /// The function `is_signer` returns a boolean value indicating whether the inner object is a signer.
    fn is_signer(&self) -> bool {
        self.inner().is_signer()
    }

    /// The `chainid` function returns the chain ID.
    fn chainid(&self) -> u16 {
        self.inner().chainid()
    }

    /// Sends a transaction and returns a more higher-level response to work with on application layer.
    async fn send_transaction(&self, tx: CreateTransactionRequest) -> Result<Transaction<Self::Provider>, Error> {
        let response = self.send_transaction_without_confirm::<CreateTransactionResponse>(tx).await?;

        Ok(Transaction::new(response.tran_id, self.provider()))
    }

    /// If there is any signer middleware, will sign it first and then send it.
    async fn send_transaction_without_confirm<T: Send + DeserializeOwned>(
        &self,
        tx: CreateTransactionRequest,
    ) -> Result<T, Error> {
        self.inner().send_transaction_without_confirm(tx).await
    }

    fn sign(&self, data: &[u8]) -> Result<Signature, Error> {
        self.inner().sign(data)
    }

    fn sign_transaction(&self, tx: &CreateTransactionRequest) -> Result<Signature, Error> {
        self.inner().sign_transaction(tx)
    }

    /// Directly calls CreateTransaction JSON-RPC endpoint.
    async fn create_transaction<T: Send + DeserializeOwned>(&self, tx: CreateTransactionRequest) -> Result<T, Error> {
        self.inner().create_transaction(tx).await
    }

    /// The function `get_transaction_status` retrieves the status of a transaction identified by its hash.
    ///
    /// Arguments:
    ///
    /// * `tx_hash`: A string representing the transaction hash.
    async fn get_transaction_status(&self, tx_hash: &TxHash) -> Result<TransactionStatus, Error> {
        self.inner().get_transaction_status(tx_hash).await
    }

    /// The function `get_transaction` retrieves a transaction using its hash.
    ///
    /// Arguments:
    ///
    /// * `tx_hash`: A string representing the transaction hash.
    async fn get_transaction(&self, tx_hash: &TxHash) -> Result<GetTransactionResponse, Error> {
        self.inner().get_transaction(tx_hash).await
    }

    async fn get_balance(&self, address: &str) -> Result<BalanceResponse, Error> {
        self.inner().get_balance(address).await
    }

    async fn get_ds_block(&self, lock_num: &str) -> Result<DsBlock, Error> {
        self.inner().get_ds_block(lock_num).await
    }

    async fn ds_block_listing(&self, max: u32) -> Result<BlockList, Error> {
        self.inner().ds_block_listing(max).await
    }
    async fn get_tx_block(&self, block_num: &str) -> Result<TxBlock, Error> {
        self.inner().get_tx_block(block_num).await
    }
    async fn tx_block_listing(&self, max: u32) -> Result<BlockList, Error> {
        self.inner().tx_block_listing(max).await
    }
    async fn get_miner_info(&self, ds_block_number: &str) -> Result<MinerInfo, Error> {
        self.inner().get_miner_info(ds_block_number).await
    }

    async fn get_blockchain_info(&self) -> Result<BlockchainInfo, Error> {
        self.inner().get_blockchain_info().await
    }

    async fn get_sharding_structure(&self) -> Result<ShardingStructure, Error> {
        self.inner().get_sharding_structure().await
    }

    async fn get_latest_ds_block(&self) -> Result<DsBlock, Error> {
        self.inner().get_latest_ds_block().await
    }

    async fn get_num_ds_blocks(&self) -> Result<String, Error> {
        self.inner().get_num_ds_blocks().await
    }

    async fn get_ds_block_rate(&self) -> Result<f32, Error> {
        self.inner().get_ds_block_rate().await
    }

    async fn get_latest_tx_block(&self) -> Result<TxBlock, Error> {
        self.inner().get_latest_tx_block().await
    }

    async fn get_num_tx_blocks(&self) -> Result<String, Error> {
        self.inner().get_num_tx_blocks().await
    }

    async fn get_tx_block_rate(&self) -> Result<f32, Error> {
        self.inner().get_tx_block_rate().await
    }

    async fn get_num_transactions(&self) -> Result<String, Error> {
        self.inner().get_num_transactions().await
    }

    async fn get_transaction_rate(&self) -> Result<f32, Error> {
        self.inner().get_transaction_rate().await
    }

    async fn get_current_mini_epoch(&self) -> Result<String, Error> {
        self.inner().get_current_mini_epoch().await
    }

    async fn get_current_ds_epoch(&self) -> Result<String, Error> {
        self.inner().get_current_ds_epoch().await
    }

    async fn get_prev_difficulty(&self) -> Result<u32, Error> {
        self.inner().get_prev_difficulty().await
    }

    async fn get_prev_ds_difficulty(&self) -> Result<u32, Error> {
        self.inner().get_prev_ds_difficulty().await
    }

    async fn get_total_coin_supply(&self) -> Result<String, Error> {
        self.inner().get_total_coin_supply().await
    }

    async fn get_recent_transactions(&self) -> Result<TxList, Error> {
        self.inner().get_recent_transactions().await
    }

    async fn get_transactions_for_tx_block(&self, tx_block: &str) -> Result<Vec<Vec<String>>, Error> {
        self.inner().get_transactions_for_tx_block(tx_block).await
    }

    async fn get_txn_bodies_for_tx_block_ex(&self, tx_block: &str, page_num: &str) -> Result<TxnBodiesForTxBlockEx, Error> {
        self.inner().get_txn_bodies_for_tx_block_ex(tx_block, page_num).await
    }

    async fn get_txn_bodies_for_tx_block(&self, tx_block: &str) -> Result<Vec<GetTransactionResponse>, Error> {
        self.inner().get_txn_bodies_for_tx_block(tx_block).await
    }

    async fn get_transactions_for_tx_block_ex(&self, tx_block: &str, page_num: &str) -> Result<TransactionsForTxBlockEx, Error> {
        self.inner().get_transactions_for_tx_block_ex(tx_block, page_num).await
    }

    async fn get_num_txns_tx_epoch(&self, epoch: &str) -> Result<String, Error> {
        self.inner().get_num_txns_tx_epoch(epoch).await
    }

    async fn get_num_txns_ds_epoch(&self, epoch: &str) -> Result<String, Error> {
        self.inner().get_num_txns_ds_epoch(epoch).await
    }

    async fn get_minimum_gas_price(&self) -> Result<String, Error> {
        self.inner().get_minimum_gas_price().await
    }

    async fn get_smart_contracts(&self, owner: &ZilAddress) -> Result<SmartContracts, Error> {
        self.inner().get_smart_contracts(owner).await
    }

    async fn get_contract_address_from_transaction_id(&self, tx_hash: &TxHash) -> Result<String, Error> {
        self.inner().get_contract_address_from_transaction_id(tx_hash).await
    }

    async fn get_smart_contract_code(&self, contract_address: &ZilAddress) -> Result<SmartContractCode, Error> {
        self.inner().get_smart_contract_code(contract_address).await
    }

    async fn get_smart_contract_init(&self, contract_address: &ZilAddress) -> Result<Vec<EventParam>, Error> {
        self.inner().get_smart_contract_init(contract_address).await
    }

    async fn get_smart_contract_state<T: Send + DeserializeOwned>(&self, contract_address: &ZilAddress) -> Result<T, Error> {
        self.inner().get_smart_contract_state(contract_address).await
    }

    async fn get_smart_contract_sub_state(
        &self,
        contract_address: &ZilAddress,
        variable_name: &str,
        indices: &[&str],
    ) -> Result<serde_json::Value, Error> {
        self.inner()
            .get_smart_contract_sub_state(contract_address, variable_name, indices)
            .await
    }

    async fn get_state_proof(
        &self,
        contract_address: &ZilAddress,
        hash: &str,
        tx_block: &str,
    ) -> Result<serde_json::Value, Error> {
        self.inner().get_state_proof(contract_address, hash, tx_block).await
    }
}
