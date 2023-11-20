use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BalanceResponse {
    pub nonce: u64,
    pub balance: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateTransactionResponse {
    #[serde(rename = "TranID")]
    pub tran_id: String,

    #[serde(rename = "Info")]
    pub info: String,
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
