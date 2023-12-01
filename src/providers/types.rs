use std::fmt::Display;

use serde::{Deserialize, Serializer};
use serde_aux::field_attributes::deserialize_number_from_string;

use crate::{crypto::ZilAddress, transaction::Version};

#[derive(Deserialize, Debug)]
pub struct BalanceResponse {
    pub nonce: u64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub balance: u128,
}

#[derive(serde::Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransactionRequest {
    pub version: Version,
    pub nonce: u64,
    pub to_addr: ZilAddress,
    #[serde(serialize_with = "to_str")]
    pub amount: u128,
    pub pub_key: Option<String>,
    #[serde(serialize_with = "to_str")]
    pub gas_price: u128,
    #[serde(serialize_with = "to_str")]
    pub gas_limit: u64,
    pub code: Option<String>,
    pub data: Option<String>,
    pub signature: Option<String>,
}

pub fn to_str<S: Serializer, T: Display>(data: T, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&data.to_string())
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
pub struct GetTransactionResponse {
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
    pub receipt: TransactionReceipt,
}

#[derive(Deserialize, Debug, Default)]
pub struct TransactionReceipt {
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
pub struct TransactionStatus {
    #[serde(rename = "ID")]
    pub id: String,
    pub _id: StatusID,
    pub amount: String,
    #[serde(rename = "epochInserted")]
    pub epoch_inserted: String,
    #[serde(rename = "epochUpdated")]
    pub epoch_updated: String,
    #[serde(rename = "gasLimit")]
    pub gas_limit: String,
    #[serde(rename = "gasPrice")]
    pub gas_price: String,
    #[serde(rename = "lastModified")]
    pub last_modified: String,
    #[serde(rename = "modificationState")]
    pub modification_state: u32,
    pub nonce: String,
    #[serde(rename = "senderAddr")]
    pub sender_addr: String,
    pub signature: String,
    pub status: u32,
    pub success: bool,
    #[serde(rename = "toAddr")]
    pub to_addr: String,
    pub version: String,
    //   statusMessage: String, // TODO: Fill it like zilliqa-js
}

#[derive(Deserialize, Debug)]
pub struct StatusID {
    #[serde(rename = "$oid")]
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct EventLogEntry {
    pub address: String,
    pub _eventname: String,
    pub params: Vec<EventParam>,
}

// TODO: DRY, This struct is like contract::Value
#[derive(Deserialize, Debug)]
pub struct EventParam {
    pub vname: String,

    #[serde(rename = "type")]
    pub r#type: String,
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
    pub transactions: Vec<GetTransactionResponse>,
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

#[derive(Deserialize, Debug)]
pub struct SmartContractCode {
    pub code: String,
}

#[derive(Deserialize, Debug)]
pub struct SmartContracts(Vec<SmartContractAddress>);

#[derive(Deserialize, Debug)]
pub struct SmartContractAddress {
    pub address: ZilAddress,
}
