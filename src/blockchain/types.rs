#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct BalanceResponse {
    pub nonce: u64,
    pub balance: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct CreateTransactionResponse {
    #[serde(rename = "TranID")]
    tran_id: String,

    #[serde(rename = "Info")]
    info: String,
}
