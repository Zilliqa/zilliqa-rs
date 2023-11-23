use std::{path::Path, sync::Arc};

use crate::{crypto::ZilAddress, middlewares::Middleware, transaction::TransactionBuilder};

use super::{error::ContractResult, Contract, Init};

pub struct Factory<T: Middleware> {
    client: Arc<T>,
}

impl<T: Middleware> Factory<T> {
    pub fn new(client: Arc<T>) -> Self {
        Self { client }
    }

    pub async fn deploy_from_file(&self, path: &Path, init: Init) -> ContractResult<Contract> {
        let contract_str = std::fs::read_to_string(path)?;

        let tx = TransactionBuilder::default()
            .to_address(ZilAddress::nil())
            .amount(0u128)
            .code(contract_str)
            .data(serde_json::to_string(&init)?)
            .gas_price(2000000000u128)
            .gas_limit(10000u64)
            .build();

        let response = self.client.deploy_contract(tx).await?;
        Ok(Contract {
            address: response.contract_address,
        })
    }
}
