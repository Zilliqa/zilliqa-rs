use std::path::Path;

use anyhow::Result;
use zilliqa_rs::{
    contract::{ContractFactory, Init, Value},
    middlewares::Middleware,
    providers::{Http, Provider},
    signers::LocalWallet,
};

const CONTRACT_PATH: &str = "tests/contracts/Timestamp.scilla";

//   {
//     vname: '_scilla_version',
//     type: 'Uint32',
//     value: '0',
//   },

#[tokio::test]
async fn deploy_contract() -> Result<()> {
    let init = Init(vec![Value::new("_scilla_version", "Uint32", "0")]);

    let tx = ContractFactory::deploy_from_file(&Path::new(CONTRACT_PATH), init)
        .await
        .unwrap();

    const END_POINT: &str = "http://localhost:5555";

    let wallet = "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba".parse::<LocalWallet>()?;

    let provider = Provider::<Http>::try_from(END_POINT)?
        .with_chain_id(1)
        .with_signer(wallet.clone());

    let response = provider.send_transaction(tx).await?;

    println!("{:?}", response);
    Ok(())
}
