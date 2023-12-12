use std::sync::Arc;

use crate::{
    crypto::ZilAddress,
    middlewares::Middleware,
    providers::GetTransactionResponse,
    transaction::{TransactionBuilder, TransactionParams},
    util::parse_zil,
    Error,
};

use super::{Transition, Value};

#[derive(Default, Debug)]
pub struct TransitionCall<T: Middleware> {
    name: String,
    contract_address: ZilAddress,
    args: Option<Vec<Value>>,
    overridden_params: TransactionParams,
    client: Arc<T>,
}

impl<T: Middleware> TransitionCall<T> {
    pub fn new(name: &str, contract_address: &ZilAddress, client: Arc<T>) -> Self {
        Self {
            name: name.to_string(),
            client,
            contract_address: contract_address.clone(),
            args: None,
            overridden_params: Default::default(),
        }
    }

    pub fn args(&mut self, args: Vec<Value>) -> &mut Self {
        self.args = Some(args);
        self
    }

    pub fn overridden_params(&mut self, overridden_params: TransactionParams) -> &mut Self {
        self.overridden_params = overridden_params;
        self
    }

    pub fn nonce(&mut self, nonce: u64) -> &mut Self {
        self.overridden_params.nonce = Some(nonce);
        self
    }

    pub fn amount(&mut self, amount: u128) -> &mut Self {
        self.overridden_params.amount = Some(amount);
        self
    }

    pub fn to_address(&mut self, to_addr: ZilAddress) -> &mut Self {
        self.overridden_params.to_addr = Some(to_addr);
        self
    }

    pub async fn call(&self) -> Result<GetTransactionResponse, Error> {
        let tx = TransactionBuilder::from(self.overridden_params.clone())
            .gas_price_if_none(parse_zil("0.002")?)
            .gas_limit_if_none(10000u64)
            .to_address(self.contract_address.clone())
            .data(serde_json::to_string(&Transition {
                tag: self.name.clone(),
                params: self.args.clone().unwrap_or_default(),
            })?)
            .build();

        let tx = self.client.send_transaction(tx).await?;
        tx.confirm().await
    }
}
