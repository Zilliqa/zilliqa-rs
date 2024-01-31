//! JSON-RPC related data types.

use std::fmt;

use primitive_types::H160;
use prost::Message;
use serde::{Deserialize, Serializer};
use serde_aux::field_attributes::deserialize_number_from_string;

use super::{proto, TxHash, ZilAddress};
use crate::{contract::ScillaVariable, transaction::Version};

#[derive(Debug)]
pub enum RPCMethod {
    // Network-related methods
    GetNetworkId,

    // Blockchain-related methods
    GetNodeType,
    GetBlockchainInfo,
    GetShardingStructure,
    GetDsBlock,
    GetDsBlockVerbose,
    GetLatestDsBlock,
    GetNumDsBlocks,
    GetDsBlockRate,
    DsBlockListing,
    GetTxBlock,
    GetLatestTxBlock,
    GetNumTxBlocks,
    GetTxBlockRate,
    TxBlockListing,
    GetNumTransactions,
    GetTransactionRate,
    GetCurrentMiniEpoch,
    GetCurrentDsEpoch,
    GetPrevDifficulty,
    GetPrevDsDifficulty,
    GetTotalCoinSupply,
    GetMinerInfo,
    GetNumPeers,

    // Transaction-related methods
    CreateTransaction,
    GetTransaction,
    GetSoftConfirmedTransaction,
    GetTransactionStatus,
    GetRecentTransactions,
    GetTransactionsForTxBlock,
    GetTransactionsForTxBlockEx,
    GetTxnBodiesForTxBlock,
    GetTxnBodiesForTxBlockEx,
    GetNumTxnsTxEpoch,
    GetNumTxnsDsEpoch,
    GetMinimumGasPrice,

    // Contract-related methods
    GetContractAddressFromTransactionId,
    GetSmartContracts,
    GetSmartContractCode,
    GetSmartContractInit,
    GetSmartContractState,
    GetSmartContractSubState,
    GetStateProof,

    // Account-related methods
    GetBalance,
}

impl fmt::Display for RPCMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DsBlockListing => write!(f, "DSBlockListing"),
            Self::GetNumDsBlocks => write!(f, "GetNumDSBlocks"),
            Self::GetDsBlockRate => write!(f, "GetDSBlockRate"),
            Self::GetCurrentDsEpoch => write!(f, "GetCurrentDSEpoch"),
            Self::GetPrevDsDifficulty => write!(f, "GetPrevDSDifficulty"),
            Self::GetNumTxnsDsEpoch => write!(f, "GetNumTxnsDSEpoch"),
            Self::GetContractAddressFromTransactionId => {
                write!(f, "GetContractAddressFromTransactionID")
            }
            _ => fmt::Debug::fmt(self, f),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct BalanceResponse {
    pub nonce: u64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub balance: u128,
}

#[derive(serde::Serialize, Default, Debug, Clone)]
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

impl CreateTransactionRequest {
    pub fn proto_encode(&self, sender_pubkey: proto::ByteArray) -> Vec<u8> {
        let to_addr: H160 = self.to_addr.parse().unwrap();
        let proto = proto::ProtoTransactionCoreInfo {
            version: self.version.pack(),
            toaddr: to_addr.as_bytes().to_vec(),
            senderpubkey: Some(sender_pubkey),
            amount: Some(self.amount.to_be_bytes().to_vec().into()),
            gasprice: Some(self.gas_price.to_be_bytes().to_vec().into()),
            gaslimit: self.gas_limit,
            oneof2: Some(proto::Nonce::Nonce(self.nonce)),
            //TODO: Remove clones
            oneof8: self.code.clone().map(|code| proto::Code::Code(code.as_bytes().to_vec())),
            oneof9: self.data.clone().map(|data| proto::Data::Data(data.as_bytes().to_vec())),
        };
        proto.encode_to_vec()
    }
}

pub fn to_str<S: Serializer, T: fmt::Display>(data: T, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&data.to_string())
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateTransactionResponse {
    #[serde(rename = "TranID")]
    pub tran_id: TxHash,

    #[serde(rename = "Info")]
    pub info: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DeployContractResponse {
    #[serde(flatten)]
    pub response: CreateTransactionResponse,

    #[serde(rename = "ContractAddress")]
    pub contract_address: ZilAddress,
}

// FIXME: Why #[serde(rename_all = "PascalCase")] does not work?!
#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Debug, Clone)]
pub struct ShardingStructure {
    #[serde(rename = "NumPeers")]
    pub num_peers: Vec<u32>,
}

#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Debug, Clone)]
pub struct DsBlock {
    pub header: DsBlockHeader,
    pub signature: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DsBlockHeaderVerbose {
    #[serde(flatten)]
    pub header: DsBlockHeader,
    #[serde(rename = "CommitteeHash")]
    pub committee_hash: String,
    #[serde(rename = "EpochNum")]
    pub epoch_num: String,
    #[serde(rename = "MembersEjected")]
    pub members_ejected: Vec<String>,
    #[serde(rename = "PoWWinnersIP")]
    pub pow_winners_ip: Vec<PoWWinnersIP>,
    #[serde(rename = "ReservedField")]
    pub reserved_field: String,
    #[serde(rename = "SWInfo")]
    pub sw_info: SWInfo,
    #[serde(rename = "Version")]
    pub version: u16,
    #[serde(rename = "ShardingHash")]
    pub sharding_hash: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SWInfo {
    #[serde(rename = "Scilla")]
    pub scilla: (u16, u16, u16, String, u16),
    #[serde(rename = "Zilliqa")]
    pub zilliqa: (u16, u16, u16, String, u16),
}

#[derive(Deserialize, Debug, Clone)]
pub struct DsBlockVerbose {
    pub header: DsBlockHeaderVerbose,
    pub signature: String,
    #[serde(rename = "B1")]
    pub b1: Vec<bool>,
    #[serde(rename = "B2")]
    pub b2: Vec<bool>,
    #[serde(rename = "CS1")]
    pub cs1: String,
    #[serde(rename = "PrevDSHash")]
    pub prev_ds_hash: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PoWWinnersIP {
    #[serde(rename = "IP")]
    pub ip: String,
    pub port: u16,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BlockShort {
    #[serde(rename = "BlockNum")]
    pub block_num: u32,
    #[serde(rename = "Hash")]
    pub hash: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BlockList {
    pub data: Vec<BlockShort>,
    #[serde(rename = "maxPages")]
    pub max_pages: u32,
}

#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Debug, Clone)]
pub struct MicroBlockInfo {
    #[serde(rename = "MicroBlockHash")]
    pub micro_block_hash: String,
    #[serde(rename = "MicroBlockShardId")]
    pub micro_block_shard_id: u32,
    #[serde(rename = "MicroBlockTxnRootHash")]
    pub micro_block_txn_root_hash: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TxBlockBody {
    #[serde(rename = "BlockHash")]
    pub block_hash: String,
    #[serde(rename = "HeaderSign")]
    pub header_sign: String,
    #[serde(rename = "MicroBlockInfos")]
    pub micro_block_infos: Vec<MicroBlockInfo>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TxBlock {
    pub body: TxBlockBody,
    pub header: TxBlockHeader,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TxList {
    pub number: u32,
    #[serde(rename = "TxnHashes")]
    pub txn_hashes: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MinerInfo {
    pub dscommittee: Vec<String>,
    pub shards: Vec<ShardInfo>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ShardInfo {
    pub nodes: Vec<String>,
    pub size: usize,
}

#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Debug, Default, Clone)]
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

impl TransactionReceipt {
    pub fn event_log(&self, event_name: &str) -> Option<&EventLogEntry> {
        if let Some(ref event_logs) = self.event_logs {
            event_logs.iter().find(|entry| entry._eventname == event_name)
        } else {
            None
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Debug, Clone)]
pub struct StatusID {
    #[serde(rename = "$oid")]
    pub id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EventLogEntry {
    pub address: String,
    pub _eventname: String,
    pub params: Vec<ScillaVariable>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ExceptionEntry {
    pub line: u32,
    pub message: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TransitionEntry {
    pub accepted: bool,
    pub addr: String,
    pub depth: usize,
    pub msg: TransitionMsg,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TransitionMsg {
    pub _amount: String,
    pub _recipient: String,
    pub _tag: String,
    pub params: Vec<ScillaVariable>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TxnBodiesForTxBlockEx {
    #[serde(rename = "CurrPage")]
    pub curr_page: u32,
    #[serde(rename = "NumPages")]
    pub num_pages: u32,
    #[serde(rename = "Transactions")]
    pub transactions: Vec<GetTransactionResponse>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TransactionsForTxBlockEx {
    #[serde(rename = "CurrPage")]
    pub curr_page: u32,
    #[serde(rename = "NumPages")]
    pub num_pages: u32,
    #[serde(rename = "Transactions")]
    pub transactions: Vec<Vec<String>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SmartContractCode {
    pub code: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SmartContracts(Vec<SmartContractAddress>);

#[derive(Deserialize, Debug, Clone)]
pub struct SmartContractAddress {
    pub address: ZilAddress,
}
