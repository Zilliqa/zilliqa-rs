use std::{path::Path, sync::Arc};

use anyhow::Result;
use zilliqa_rs::{
    contract::{self, ContractFactory, Init, Value},
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

    let init = Init(vec![Value::new_from_str("_scilla_version", "Uint32", "0")]);

    let contract = factory.deploy_from_file(&Path::new(CONTRACT_PATH), init, None).await.unwrap();

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
        Value::new_from_str("_scilla_version", "Uint32", "0"),
        Value::new_from_str("owner", "ByStr20", &wallet.address.to_string()),
    ]);

    let contract = factory.deploy_from_file(&Path::new(CONTRACT_PATH), init, None).await.unwrap();

    println!("addr: {:?}", contract);
    Ok(())
}

#[tokio::test]
async fn deploy_from_string() -> Result<()> {
    const CONTRACT_CODE: &str = include_str!("contracts/HelloWorld.scilla");
    const END_POINT: &str = "http://localhost:5555";

    let wallet = "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba".parse::<LocalWallet>()?;

    let provider = Provider::<Http>::try_from(END_POINT)?
        .with_chain_id(1)
        .with_signer(wallet.clone());

    let provider = Arc::new(provider);
    let factory = ContractFactory::new(provider.clone());

    let init = Init(vec![
        Value::new_from_str("_scilla_version", "Uint32", "0"),
        Value::new_from_str("owner", "ByStr20", &wallet.address.to_string()),
    ]);

    let _contract = factory.deploy_str(CONTRACT_CODE.to_string(), init, None).await.unwrap();

    Ok(())
}

#[tokio::test]
async fn call_a_param_less_transition() -> Result<()> {
    const CONTRACT_PATH: &str = "tests/contracts/HelloWorld.scilla";
    const END_POINT: &str = "http://localhost:5555";

    let wallet = "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba".parse::<LocalWallet>()?;

    let provider = Provider::<Http>::try_from(END_POINT)?
        .with_chain_id(1)
        .with_signer(wallet.clone());

    let provider = Arc::new(provider);
    let factory = ContractFactory::new(provider.clone());

    let init = Init(vec![
        Value::new_from_str("_scilla_version", "Uint32", "0"),
        Value::new_from_str("owner", "ByStr20", &wallet.address.to_string()),
    ]);

    let contract = factory.deploy_from_file(&Path::new(CONTRACT_PATH), init, None).await.unwrap();

    let response = contract.call("getHello", vec![]).await?;

    println!("{response:?}");

    Ok(())
}

#[tokio::test]
async fn call_transition_with_single_string_param() -> Result<()> {
    const CONTRACT_PATH: &str = "tests/contracts/HelloWorld.scilla";
    const END_POINT: &str = "http://localhost:5555";

    let wallet = "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba".parse::<LocalWallet>()?;

    let provider = Provider::<Http>::try_from(END_POINT)?
        .with_chain_id(1)
        .with_signer(wallet.clone());

    let provider = Arc::new(provider);
    let factory = ContractFactory::new(provider.clone());

    let init = Init(vec![
        Value::new_from_str("_scilla_version", "Uint32", "0"),
        Value::new_from_str("owner", "ByStr20", &wallet.address.to_string()),
    ]);

    let contract = factory.deploy_from_file(&Path::new(CONTRACT_PATH), init, None).await.unwrap();

    let response = contract
        .call("setHello", vec![Value::new_from_str("msg", "String", "heellleeo")])
        .await?;

    println!("{response:?}");

    Ok(())
}

#[tokio::test]
async fn call_a_param_less_transition_though_the_rust_binding() -> Result<()> {
    const CONTRACT_PATH: &str = "tests/contracts/HelloWorld.scilla";
    const END_POINT: &str = "http://localhost:5555";

    let wallet = "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba".parse::<LocalWallet>()?;

    let provider = Provider::<Http>::try_from(END_POINT)?
        .with_chain_id(1)
        .with_signer(wallet.clone());

    let provider = Arc::new(provider);
    let factory = ContractFactory::new(provider.clone());

    let init = Init(vec![
        Value::new_from_str("_scilla_version", "Uint32", "0"),
        Value::new_from_str("owner", "ByStr20", &wallet.address.to_string()),
    ]);

    let contract = factory.deploy_from_file(&Path::new(CONTRACT_PATH), init, None).await.unwrap();

    let contract = contract::HelloWorld::new(contract);

    let response = contract.get_hello().await?;

    println!("{response:?}");

    Ok(())
}

#[tokio::test]
async fn deploy_a_paramless_contract_through_the_rust_binding() -> Result<()> {
    const END_POINT: &str = "http://localhost:5555";

    let wallet = "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba".parse::<LocalWallet>()?;

    let provider = Provider::<Http>::try_from(END_POINT)?
        .with_chain_id(1)
        .with_signer(wallet.clone());

    let provider = Arc::new(provider);

    let _contract = contract::Timestamp::deploy(provider).await?;

    Ok(())
}

#[tokio::test]
async fn deploy_a_one_param_contract_through_the_rust_binding() -> Result<()> {
    const END_POINT: &str = "http://localhost:5555";

    let wallet = "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba".parse::<LocalWallet>()?;

    let provider = Provider::<Http>::try_from(END_POINT)?
        .with_chain_id(1)
        .with_signer(wallet.clone());

    let contract = contract::HelloWorld::deploy(Arc::new(provider), wallet.address.to_string()).await?;

    let response = contract.get_hello().await?;

    println!("{response:?}");

    let state = contract.get_state().await?;
    let hello = contract.welcome_msg().await?;
    assert_eq!(hello, state.welcome_msg);

    Ok(())
}

#[tokio::test]
async fn call_transition_with_single_string_param_using_rust_binding() -> Result<()> {
    const END_POINT: &str = "http://localhost:5555";

    let wallet = "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba".parse::<LocalWallet>()?;

    let provider = Provider::<Http>::try_from(END_POINT)?
        .with_chain_id(1)
        .with_signer(wallet.clone());

    let contract = contract::HelloWorld::deploy(Arc::new(provider), wallet.address.to_string()).await?;

    let response = contract.set_hello("heellleeo".to_string()).await?;

    println!("{response:?}");

    Ok(())
}
