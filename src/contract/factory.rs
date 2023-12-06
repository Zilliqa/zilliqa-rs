use std::{path::Path, sync::Arc};

use crate::{
    crypto::ZilAddress,
    middlewares::Middleware,
    transaction::{TransactionBuilder, TransactionParams},
    util::parse_zil,
    Error,
};

use super::{BaseContract, Init};

pub struct Factory<T: Middleware> {
    client: Arc<T>,
}

impl<T: Middleware> Factory<T> {
    pub fn new(client: Arc<T>) -> Self {
        Self { client }
    }

    pub async fn deploy_from_file(
        &self,
        path: &Path,
        init: Init,
        overridden_params: Option<TransactionParams>,
    ) -> Result<BaseContract<T>, Error> {
        let contract_code = std::fs::read_to_string(path)?;
        self.deploy_str(contract_code, init, overridden_params).await
    }

    pub async fn deploy_str(
        &self,
        contract_code: String,
        init: Init,
        overridden_params: Option<TransactionParams>,
    ) -> Result<BaseContract<T>, Error> {
        let tx = overridden_params
            .map(TransactionBuilder::from)
            .unwrap_or_default()
            .to_address(ZilAddress::nil())
            .amount_if_none(0_u128)
            .code(contract_code)
            .data(serde_json::to_string(&init)?)
            .gas_price_if_none(parse_zil("0.002")?)
            .gas_limit_if_none(10000u64)
            .build();

        let response = self.client.deploy_contract(tx).await?;
        Ok(BaseContract {
            address: response.contract_address,
            client: self.client.clone(),
        })
    }
}
