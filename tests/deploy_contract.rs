use std::{path::Path, sync::Arc};

use anyhow::Result;
use zilliqa_rs::{
    contract::{ContractFactory, Init, Value},
    providers::{Http, Provider},
    signers::LocalWallet,
};

#[tokio::test]
async fn deploy_contract_without_constructor_parameter() -> Result<()> {
    const CONTRACT_PATH: &str = "tests/contracts/Timestamp.scilla";
    const END_POINT: &str = "http://localhost:5555";

    let wallet = "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba".parse::<LocalWallet>()?;

    let provider = Provider::<Http>::try_from(END_POINT)?
        .with_chain_id(1)
        .with_signer(wallet.clone());

    let factory = ContractFactory::new(Arc::new(provider));

    let init = Init(vec![Value::new("_scilla_version", "Uint32", "0")]);

    let contract = factory.deploy_from_file(&Path::new(CONTRACT_PATH), init).await.unwrap();

    println!("addr: {:?}", contract);
    Ok(())
}

#[tokio::test]
async fn deploy_contract_with_constructor_parameter() -> Result<()> {
    const CONTRACT_PATH: &str = "tests/contracts/HelloWorld.scilla";
    const END_POINT: &str = "http://localhost:5555";

    let wallet = "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba".parse::<LocalWallet>()?;

    let provider = Provider::<Http>::try_from(END_POINT)?
        .with_chain_id(1)
        .with_signer(wallet.clone());

    let factory = ContractFactory::new(Arc::new(provider));

    let init = Init(vec![
        Value::new("_scilla_version", "Uint32", "0"),
        Value::new("owner", "ByStr20", &wallet.address.to_string()),
    ]);

    let contract = factory.deploy_from_file(&Path::new(CONTRACT_PATH), init).await.unwrap();

    println!("addr: {:?}", contract);
    Ok(())
}
