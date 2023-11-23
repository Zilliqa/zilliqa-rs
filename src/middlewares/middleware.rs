use crate::{
    crypto::Signature,
    providers::{types::*, JsonRpcClient, Provider},
    transaction::Transaction,
};
use async_trait::async_trait;

use super::MiddlewareResult;

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

    // /// Returns the currently configured chain id, a value used in replay-protected
    // /// transaction signing as introduced by EIP-155.
    // /// This returns true if either the middleware stack contains a `SignerMiddleware`, or the
    // /// JSON-RPC provider has an unlocked key that can sign using the `eth_sign` call. If none of
    // /// the above conditions are met, then the middleware stack is not capable of signing data.
    fn is_signer(&self) -> bool {
        self.inner().is_signer()
    }

    // /// Signs data using a specific account. This account needs to be unlocked,
    // /// or the middleware stack must contain a `SignerMiddleware`
    // async fn sign<T: Into<Bytes> + Send + Sync>(&self, data: T, from: &Address) -> Result<Signature, Self::Error> {
    //     self.inner().sign(data, from).await.map_err(MiddlewareError::from_err)
    // }
    fn get_chainid(&self) -> u16 {
        self.inner().get_chainid()
    }

    async fn send_transaction(&self, tx: Transaction) -> MiddlewareResult<CreateTransactionResponse> {
        self.inner().send_transaction(tx).await
    }

    fn sign_transaction(&self, tx: &Transaction) -> MiddlewareResult<Signature> {
        self.inner().sign_transaction(tx)
    }

    async fn create_transaction(&self, tx: Transaction) -> MiddlewareResult<CreateTransactionResponse> {
        self.inner().create_transaction(tx).await
    }

    async fn get_balance(&self, address: &str) -> MiddlewareResult<BalanceResponse> {
        self.inner().get_balance(address).await
    }

    async fn get_ds_block(&self, lock_num: &str) -> MiddlewareResult<DsBlock> {
        self.inner().get_ds_block(lock_num).await
    }

    async fn ds_block_listing(&self, max: u32) -> MiddlewareResult<BlockList> {
        self.inner().ds_block_listing(max).await
    }
    async fn get_tx_block(&self, block_num: &str) -> MiddlewareResult<TxBlock> {
        self.inner().get_tx_block(block_num).await
    }
    async fn tx_block_listing(&self, max: u32) -> MiddlewareResult<BlockList> {
        self.inner().tx_block_listing(max).await
    }
    async fn get_miner_info(&self, ds_block_number: &str) -> MiddlewareResult<MinerInfo> {
        self.inner().get_miner_info(ds_block_number).await
    }

    async fn get_blockchain_info(&self) -> MiddlewareResult<BlockchainInfo> {
        self.inner().get_blockchain_info().await
    }

    async fn get_sharding_structure(&self) -> MiddlewareResult<ShardingStructure> {
        self.inner().get_sharding_structure().await
    }

    async fn get_latest_ds_block(&self) -> MiddlewareResult<DsBlock> {
        self.inner().get_latest_ds_block().await
    }

    async fn get_num_ds_blocks(&self) -> MiddlewareResult<String> {
        self.inner().get_num_ds_blocks().await
    }

    async fn get_ds_block_rate(&self) -> MiddlewareResult<f32> {
        self.inner().get_ds_block_rate().await
    }

    async fn get_latest_tx_block(&self) -> MiddlewareResult<TxBlock> {
        self.inner().get_latest_tx_block().await
    }

    async fn get_num_tx_blocks(&self) -> MiddlewareResult<String> {
        self.inner().get_num_tx_blocks().await
    }

    async fn get_tx_block_rate(&self) -> MiddlewareResult<f32> {
        self.inner().get_tx_block_rate().await
    }

    async fn get_num_transactions(&self) -> MiddlewareResult<String> {
        self.inner().get_num_transactions().await
    }

    async fn get_transaction_rate(&self) -> MiddlewareResult<f32> {
        self.inner().get_transaction_rate().await
    }

    async fn get_current_mini_epoch(&self) -> MiddlewareResult<String> {
        self.inner().get_current_mini_epoch().await
    }

    async fn get_current_ds_epoch(&self) -> MiddlewareResult<String> {
        self.inner().get_current_ds_epoch().await
    }

    async fn get_prev_difficulty(&self) -> MiddlewareResult<u32> {
        self.inner().get_prev_difficulty().await
    }

    async fn get_prev_ds_difficulty(&self) -> MiddlewareResult<u32> {
        self.inner().get_prev_ds_difficulty().await
    }

    async fn get_total_coin_supply(&self) -> MiddlewareResult<String> {
        self.inner().get_total_coin_supply().await
    }

    async fn get_recent_transactions(&self) -> MiddlewareResult<TxList> {
        self.inner().get_recent_transactions().await
    }

    async fn get_transactions_for_tx_block(&self, tx_block: &str) -> MiddlewareResult<Vec<Vec<String>>> {
        self.inner().get_transactions_for_tx_block(tx_block).await
    }

    async fn get_txn_bodies_for_tx_block_ex(&self, tx_block: &str, page_num: &str) -> MiddlewareResult<TxnBodiesForTxBlockEx> {
        self.inner().get_txn_bodies_for_tx_block_ex(tx_block, page_num).await
    }

    async fn get_txn_bodies_for_tx_block(&self, tx_block: &str) -> MiddlewareResult<Vec<TransactionObj>> {
        self.inner().get_txn_bodies_for_tx_block(tx_block).await
    }

    async fn get_transactions_for_tx_block_ex(
        &self,
        tx_block: &str,
        page_num: &str,
    ) -> MiddlewareResult<TransactionsForTxBlockEx> {
        self.inner().get_transactions_for_tx_block_ex(tx_block, page_num).await
    }

    async fn get_num_txns_tx_epoch(&self, epoch: &str) -> MiddlewareResult<String> {
        self.inner().get_num_txns_tx_epoch(epoch).await
    }

    async fn get_num_txns_ds_epoch(&self, epoch: &str) -> MiddlewareResult<String> {
        self.inner().get_num_txns_ds_epoch(epoch).await
    }

    async fn get_minimum_gas_price(&self) -> MiddlewareResult<String> {
        self.inner().get_minimum_gas_price().await
    }
}
