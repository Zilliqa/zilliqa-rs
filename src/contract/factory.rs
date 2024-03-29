use std::{path::Path, sync::Arc};

use crate::{
    core::parse_zil,
    core::{DeployContractResponse, ZilAddress},
    middlewares::Middleware,
    transaction::{Transaction, TransactionBuilder, TransactionParams},
    Error,
};

use super::{compress_contract, BaseContract, Init};

pub struct Factory<T: Middleware> {
    client: Arc<T>,
}

impl<T: Middleware> Factory<T> {
    pub fn new(client: Arc<T>) -> Self {
        Self { client }
    }

    /// The `deploy_from_file` function deploys a contract from a file path, with the option to override
    /// transaction parameters.
    ///
    /// Arguments:
    ///
    /// * `path`: The `path` parameter is a reference to a `Path` object, which represents the path to a
    /// file. It is used to specify the location of the file from which the contract code will be read.
    /// * `init`: The `init` parameter is of type `Init`. It represents the initialization parameters for
    /// the contract being deployed. The specific structure and fields of the `Init` type would depend on
    /// the contract being deployed.
    /// * `overridden_params`: `overridden_params` is an optional parameter of type `TransactionParams`. It
    /// allows you to override the default transaction parameters when deploying the contract. If you don't
    /// want to override any parameters, you can pass `None` as the value for this parameter.
    /// * `do_contract_compression`: Set it to true if you want your contract gets compressed before deployment.
    ///
    /// Returns:
    ///
    /// a Result object with a value of type `BaseContract<T>` if the operation is successful, or an Error
    /// object if there is an error.
    /// # Example
    /// ```
    /// use zilliqa_rs::providers::{Http, Provider};
    /// use zilliqa_rs::contract::ScillaVariable;
    /// use zilliqa_rs::signers::LocalWallet;
    /// use zilliqa_rs::contract::Init;
    /// use zilliqa_rs::contract::ContractFactory;
    /// use std::path::PathBuf;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     const END_POINT: &str = "http://localhost:5555";
    ///
    ///     let wallet = "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba".parse::<LocalWallet>()?;
    ///
    ///     let provider = Provider::<Http>::try_from(END_POINT)?
    ///         .with_chain_id(222)
    ///         .with_signer(wallet.clone());
    ///
    ///     let factory = ContractFactory::new(provider.into());
    ///     let init = Init(vec![ScillaVariable::new_from_str("_scilla_version", "Uint32", "0")]);
    ///     let contract = factory.deploy_from_file(&PathBuf::from("./tests/contracts/Timestamp.scilla"), init, None, false).await?;
    ///     println!("addr: {:?}", contract);
    ///     Ok(())
    /// }
    /// ```
    pub async fn deploy_from_file(
        &self,
        path: &Path,
        init: Init,
        overridden_params: Option<TransactionParams>,
        do_contract_compression: bool,
    ) -> Result<BaseContract<T>, Error> {
        let contract_code = {
            let code = std::fs::read_to_string(path)?;
            if do_contract_compression {
                compress_contract(&code)?
            } else {
                code
            }
        };
        self.deploy_str(contract_code, init, overridden_params).await
    }

    /// The `deploy_str` function deploys a contract with the given code and initialization parameters, and
    /// returns a `BaseContract` object.
    ///
    /// Arguments:
    ///
    /// * `contract_code`: A string containing the code of the contract to be deployed.
    /// * `init`: The `init` parameter is of type `Init`, which is a custom struct or enum that contains the
    /// initialization data for the contract.
    /// * `overridden_params`: `overridden_params` is an optional parameter of type
    /// `Option<TransactionParams>`. It allows the caller to provide custom transaction parameters for
    /// deploying the contract.
    ///
    /// Returns:
    ///
    /// The function `deploy_str` returns a `Result` containing either a `BaseContract<T>` or an `Error`.
    /// # Example
    /// ```
    /// use zilliqa_rs::providers::{Http, Provider};
    /// use zilliqa_rs::contract::ScillaVariable;
    /// use zilliqa_rs::signers::LocalWallet;
    /// use zilliqa_rs::contract::Init;
    /// use zilliqa_rs::contract::ContractFactory;
    /// use std::path::PathBuf;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     const END_POINT: &str = "http://localhost:5555";
    ///
    ///     let wallet = "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba".parse::<LocalWallet>()?;
    ///
    ///     let provider = Provider::<Http>::try_from(END_POINT)?
    ///         .with_chain_id(222)
    ///         .with_signer(wallet.clone());
    ///
    ///     let factory = ContractFactory::new(provider.into());
    ///     let init = Init(vec![ScillaVariable::new_from_str("_scilla_version", "Uint32", "0")]);
    ///     let contract = factory.deploy_str(include_str!("../../tests/contracts/Timestamp.scilla").to_string(), init, None).await?;
    ///     println!("addr: {:?}", contract);
    ///     Ok(())
    /// }
    /// ```
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

        let response: DeployContractResponse = self.client.send_transaction_without_confirm(tx).await?;
        let transaction = Transaction::new(response.response.tran_id, self.client.provider());
        transaction.confirm().await?;
        Ok(BaseContract {
            address: response.contract_address,
            client: self.client.clone(),
        })
    }
}
