use std::path::Path;

use crate::{
    crypto::ZilAddress,
    transaction::{Transaction, TransactionBuilder},
};

use super::{error::ContractResult, Contract, Init};

pub struct Factory;

impl Factory {
    pub async fn deploy_from_file(path: &Path, init: Init) -> ContractResult<Transaction> {
        let contract_str = std::fs::read_to_string(path)?;

        Ok(TransactionBuilder::default()
            .to_address(ZilAddress::nil())
            .amount(0u128)
            .code(contract_str)
            .data(serde_json::to_string(&init)?)
            .gas_price(2000000000u128)
            .gas_limit(500u64)
            .build())
    }
}
