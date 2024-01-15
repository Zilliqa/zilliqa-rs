mod common;

use common::TestContext;
use std::{path::PathBuf, str::FromStr, sync::Arc};
use test_context::test_context;

use anyhow::Result;
use zilliqa_rs::{
    contract::{self, ContractFactory, Init, ScillaVariable},
    providers::Provider,
    signers::LocalWallet,
};

impl TestContext {
    fn timestamp_contract(&self) -> PathBuf {
        PathBuf::from("tests/contracts/Timestamp.scilla")
    }

    fn hello_world_contract(&self) -> PathBuf {
        PathBuf::from("tests/contracts/HelloWorld.scilla")
    }
}

#[test_context(TestContext)]
#[tokio::test]
async fn deploy_contract_without_constructor_parameter(ctx: &TestContext) -> Result<()> {
    let provider = ctx.provider();

    let factory = ContractFactory::new(provider.clone());

    let init = Init(vec![ScillaVariable::new_from_str("_scilla_version", "Uint32", "0")]);

    let contract = factory.deploy_from_file(&ctx.timestamp_contract(), init, None).await.unwrap();

    println!("addr: {:?}", contract);
    Ok(())
}

#[test_context(TestContext)]
#[tokio::test]
async fn deploy_contract_with_constructor_parameter(ctx: &TestContext) -> Result<()> {
    let provider = ctx.provider();
    let factory = ContractFactory::new(provider);

    let init = Init(vec![
        ScillaVariable::new_from_str("_scilla_version", "Uint32", "0"),
        ScillaVariable::new_from_str("owner", "ByStr20", &ctx.wallet.address),
    ]);

    let contract = factory
        .deploy_from_file(&ctx.hello_world_contract(), init, None)
        .await
        .unwrap();

    println!("addr: {:?}", contract);
    Ok(())
}

#[test_context(TestContext)]
#[tokio::test]
async fn deploy_from_string(ctx: &TestContext) -> Result<()> {
    const CONTRACT_CODE: &str = include_str!("contracts/HelloWorld.scilla");
    let provider = ctx.provider();
    let factory = ContractFactory::new(provider.clone());

    let init = Init(vec![
        ScillaVariable::new_from_str("_scilla_version", "Uint32", "0"),
        ScillaVariable::new_from_str("owner", "ByStr20", &ctx.wallet.address),
    ]);

    let _contract = factory.deploy_str(CONTRACT_CODE.to_string(), init, None).await.unwrap();

    Ok(())
}

#[test_context(TestContext)]
#[tokio::test]
async fn call_a_param_less_transition(ctx: &TestContext) -> Result<()> {
    let provider = ctx.provider();
    let factory = ContractFactory::new(provider.clone());

    let init = Init(vec![
        ScillaVariable::new_from_str("_scilla_version", "Uint32", "0"),
        ScillaVariable::new_from_str("owner", "ByStr20", &ctx.wallet.address),
    ]);

    let contract = factory
        .deploy_from_file(&ctx.hello_world_contract(), init, None)
        .await
        .unwrap();

    let response = contract.call("getHello", vec![], None).await?;

    println!("{response:?}");

    Ok(())
}

#[test_context(TestContext)]
#[tokio::test]
async fn call_transition_with_single_string_param(ctx: &TestContext) -> Result<()> {
    let provider = ctx.provider();
    let factory = ContractFactory::new(provider.clone());

    let init = Init(vec![
        ScillaVariable::new_from_str("_scilla_version", "Uint32", "0"),
        ScillaVariable::new_from_str("owner", "ByStr20", &ctx.wallet.address),
    ]);

    let contract = factory
        .deploy_from_file(&ctx.hello_world_contract(), init, None)
        .await
        .unwrap();

    let response = contract
        .call(
            "setHello",
            vec![ScillaVariable::new_from_str("msg", "String", "heellleeo")],
            None,
        )
        .await?;

    println!("{response:?}");

    Ok(())
}

#[test_context(TestContext)]
#[tokio::test]
async fn call_a_param_less_transition_though_the_rust_binding(ctx: &TestContext) -> Result<()> {
    let provider = ctx.provider();
    let factory = ContractFactory::new(provider.clone());

    let init = Init(vec![
        ScillaVariable::new_from_str("_scilla_version", "Uint32", "0"),
        ScillaVariable::new_from_str("owner", "ByStr20", &ctx.wallet.address),
    ]);

    let contract = factory
        .deploy_from_file(&ctx.hello_world_contract(), init, None)
        .await
        .unwrap();

    let contract = contract::HelloWorld::new(contract);

    let response = contract.get_hello().call().await?;

    println!("{response:?}");

    Ok(())
}

#[test_context(TestContext)]
#[tokio::test]
async fn deploy_a_paramless_contract_through_the_rust_binding(ctx: &TestContext) -> Result<()> {
    let provider = ctx.provider();
    let _contract = contract::Timestamp::deploy(provider).await?;
    Ok(())
}

#[test_context(TestContext)]
#[tokio::test]
async fn deploy_a_one_param_contract_through_the_rust_binding(ctx: &TestContext) -> Result<()> {
    let provider = ctx.provider();
    // TODO: ZilAddress clone can be removed
    let contract = contract::HelloWorld::deploy(provider, ctx.wallet.address.clone()).await?;

    let response = contract.get_hello().call().await?;

    println!("{response:?}");

    let hello = contract.welcome_msg().await?;
    assert_eq!(hello, "Hello world".to_string());

    Ok(())
}

#[test_context(TestContext)]
#[tokio::test]
async fn call_transition_with_single_string_param_using_rust_binding(ctx: &TestContext) -> Result<()> {
    let provider = ctx.provider();
    let contract = contract::HelloWorld::deploy(provider, ctx.wallet.address.clone()).await?;

    let response = contract.set_hello("heellleeo".to_string()).call().await?;

    println!("{response:?}");

    Ok(())
}

#[test_context(TestContext)]
#[tokio::test]
async fn call_transition_with_a_different_signer(ctx: &TestContext) -> Result<()> {
    let wallet = LocalWallet::from_str("d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba").unwrap();
    let provider = Arc::new(
        Provider::try_from(ctx.endpoint.clone())
            .unwrap()
            .with_chain_id(ctx.chain_id)
            .with_signer(wallet.clone()),
    );
    let contract = contract::HelloWorld::deploy(provider, ctx.wallet.address.clone()).await?;

    let wallet = LocalWallet::from_str("589417286a3213dceb37f8f89bd164c3505a4cec9200c61f7c6db13a30a71b45").unwrap();
    let new_provider = Arc::new(
        Provider::try_from(ctx.endpoint.clone())
            .unwrap()
            .with_chain_id(ctx.chain_id)
            .with_signer(wallet.clone()),
    );

    let response = contract
        .set_hello("heellleeo".to_string())
        .signer(new_provider)
        .call()
        .await?;

    println!("{response:?}");

    Ok(())
}
