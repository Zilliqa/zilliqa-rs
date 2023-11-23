use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

use crate::crypto::ZilAddress;

#[derive(Deserialize, Debug)]
pub struct BalanceResponse {
    pub nonce: u64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub balance: u128,
}

#[derive(Deserialize, Debug)]
pub struct CreateTransactionResponse {
    #[serde(rename = "TranID")]
    pub tran_id: String,

    #[serde(rename = "Info")]
    pub info: String,
}

#[derive(Deserialize, Debug)]
pub struct DeployContractResponse {
    #[serde(flatten)]
    pub response: CreateTransactionResponse,

    #[serde(rename = "ContractAddress")]
    pub contract_address: ZilAddress,
}

// FIXME: Why #[serde(rename_all = "PascalCase")] does not work?!
#[derive(Deserialize, Debug)]
pub struct BlockchainInfo {
    #[serde(rename = "NumPeers")]
    pub num_peers: u32,

    #[serde(rename = "NumTxBlocks")]
    pub num_tx_blocks: String,

    #[serde(rename = "NumDSBlocks")]
    pub num_dsblocks: String,

    #[serde(rename = "NumTransactions")]
    pub num_transactions: String,

    #[serde(rename = "TransactionRate")]
    pub transaction_rate: f32,

    #[serde(rename = "TxBlockRate")]
    pub tx_block_rate: f32,

    #[serde(rename = "DSBlockRate")]
    pub dsblock_rate: f32,

    #[serde(rename = "CurrentMiniEpoch")]
    pub current_mini_epoch: String,

    #[serde(rename = "CurrentDSEpoch")]
    pub current_dsepoch: String,

    #[serde(rename = "NumTxnsDSEpoch")]
    pub num_txns_dsepoch: String,

    #[serde(rename = "NumTxnsTxEpoch")]
    pub num_txns_tx_epoch: String,

    #[serde(rename = "ShardingStructure")]
    pub sharding_structure: ShardingStructure,
}

#[derive(Deserialize, Debug)]
pub struct ShardingStructure {
    #[serde(rename = "NumPeers")]
    pub num_peers: Vec<u32>,
}

#[derive(Deserialize, Debug)]
pub struct DsBlockHeader {
    #[serde(rename = "BlockNum")]
    pub block_num: String,
    #[serde(rename = "Difficulty")]
    pub difficulty: u32,
    #[serde(rename = "DifficultyDS")]
    pub difficulty_ds: u32,
    #[serde(rename = "GasPrice")]
    pub gas_price: String,
    #[serde(rename = "LeaderPubKey")]
    pub leader_pub_key: String,
    #[serde(rename = "PoWWinners")]
    pub pow_winners: Vec<String>,
    #[serde(rename = "PrevHash")]
    pub prev_hash: String,
    #[serde(rename = "Timestamp")]
    pub timestamp: String,
}

#[derive(Deserialize, Debug)]
pub struct DsBlock {
    pub header: DsBlockHeader,
    pub signature: String,
}

#[derive(Deserialize, Debug)]
pub struct BlockShort {
    #[serde(rename = "BlockNum")]
    pub block_num: u32,
    #[serde(rename = "Hash")]
    pub hash: String,
}

#[derive(Deserialize, Debug)]
pub struct BlockList {
    pub data: Vec<BlockShort>,
    #[serde(rename = "maxPages")]
    pub max_pages: u32,
}

#[derive(Deserialize, Debug)]
pub struct TxBlockHeader {
    #[serde(rename = "BlockNum")]
    pub block_num: String,
    #[serde(rename = "DSBlockNum")]
    pub dsblock_num: String,
    #[serde(rename = "GasLimit")]
    pub gas_limit: String,
    #[serde(rename = "GasUsed")]
    pub gas_used: String,
    #[serde(rename = "MbInfoHash")]
    pub mb_info_hash: String,
    #[serde(rename = "MinerPubKey")]
    pub miner_pub_key: String,
    #[serde(rename = "NumMicroBlocks")]
    pub num_micro_blocks: u32,
    #[serde(rename = "NumPages")]
    pub num_pages: u32,
    #[serde(rename = "NumTxns")]
    pub num_txns: u32,
    #[serde(rename = "PrevBlockHash")]
    pub prev_block_hash: String,
    #[serde(rename = "Rewards")]
    pub rewards: String,
    #[serde(rename = "StateDeltaHash")]
    pub state_delta_hash: String,
    #[serde(rename = "StateRootHash")]
    pub state_root_hash: String,
    #[serde(rename = "Timestamp")]
    pub timestamp: String,
    #[serde(rename = "TxnFees")]
    pub txn_fees: String,
    #[serde(rename = "Version")]
    pub version: u32,
}

#[derive(Deserialize, Debug)]
pub struct MicroBlockInfo {
    #[serde(rename = "MicroBlockHash")]
    pub micro_block_hash: String,
    #[serde(rename = "MicroBlockShardId")]
    pub micro_block_shard_id: u32,
    #[serde(rename = "MicroBlockTxnRootHash")]
    pub micro_block_txn_root_hash: String,
}

#[derive(Deserialize, Debug)]
pub struct TxBlockBody {
    #[serde(rename = "BlockHash")]
    pub block_hash: String,
    #[serde(rename = "HeaderSign")]
    pub header_sign: String,
    #[serde(rename = "MicroBlockInfos")]
    pub micro_block_infos: Vec<MicroBlockInfo>,
}

#[derive(Deserialize, Debug)]
pub struct TxBlock {
    pub body: TxBlockBody,
    pub header: TxBlockHeader,
}

#[derive(Deserialize, Debug)]
pub struct TxList {
    pub number: u32,
    #[serde(rename = "TxnHashes")]
    pub txn_hashes: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct MinerInfo {
    pub dscommittee: Vec<String>,
    pub shards: Vec<ShardInfo>,
}

#[derive(Deserialize, Debug)]
pub struct ShardInfo {
    pub nodes: Vec<String>,
    pub size: usize,
}

#[derive(Deserialize, Debug)]
pub struct TransactionObj {
    #[serde(rename = "ID")]
    pub id: String,
    pub version: String,
    pub nonce: String,
    #[serde(rename = "toAddr")]
    pub to_addr: String,
    pub amount: String,
    pub code: Option<String>,
    pub data: Option<String>,
    #[serde(rename = "gasPrice")]
    pub gas_price: String,
    #[serde(rename = "gasLimit")]
    pub gas_limit: String,
    pub signature: String,
    #[serde(rename = "senderPubKey")]
    pub sender_pub_key: String,
    pub receipt: TransactionReceiptObj,
}

#[derive(Deserialize, Debug)]
pub struct TransactionReceiptObj {
    pub accepted: Option<bool>,
    pub cumulative_gas: String,
    pub epoch_num: String,
    pub event_logs: Option<Vec<EventLogEntry>>,
    pub exceptions: Option<Vec<ExceptionEntry>>,
    pub success: bool,
    pub transitions: Option<Vec<TransitionEntry>>,
    pub errors: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct EventLogEntry {
    pub address: String,
    pub _eventname: String,
    pub params: Vec<EventParam>,
}

#[derive(Deserialize, Debug)]
pub struct EventParam {
    pub vname: String,
    pub _type: String,
    pub value: String,
}

#[derive(Deserialize, Debug)]
pub struct ExceptionEntry {
    pub line: u32,
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct TransitionEntry {
    pub accepted: bool,
    pub addr: String,
    pub depth: usize,
    pub msg: TransitionMsg,
}

#[derive(Deserialize, Debug)]
pub struct TransitionMsg {
    pub _amount: String,
    pub _recipient: String,
    pub _tag: String,
    pub params: Vec<EventParam>,
}

#[derive(Deserialize, Debug)]
pub struct TxnBodiesForTxBlockEx {
    #[serde(rename = "CurrPage")]
    pub curr_page: u32,
    #[serde(rename = "NumPages")]
    pub num_pages: u32,
    #[serde(rename = "Transactions")]
    pub transactions: Vec<TransactionObj>,
}

#[derive(Deserialize, Debug)]
pub struct TransactionsForTxBlockEx {
    #[serde(rename = "CurrPage")]
    pub curr_page: u32,
    #[serde(rename = "NumPages")]
    pub num_pages: u32,
    #[serde(rename = "Transactions")]
    pub transactions: Vec<Vec<String>>,
}
