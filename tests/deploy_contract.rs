mod common;

use common::TestContext;
use std::path::PathBuf;
use test_context::test_context;

use anyhow::Result;
use zilliqa_rs::contract::{self, ContractFactory, Init, Value};

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

    let init = Init(vec![Value::new_from_str("_scilla_version", "Uint32", "0")]);

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
        Value::new_from_str("_scilla_version", "Uint32", "0"),
        Value::new_from_str("owner", "ByStr20", &ctx.wallet.address.to_string()),
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
        Value::new_from_str("_scilla_version", "Uint32", "0"),
        Value::new_from_str("owner", "ByStr20", &ctx.wallet.address.to_string()),
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
        Value::new_from_str("_scilla_version", "Uint32", "0"),
        Value::new_from_str("owner", "ByStr20", &ctx.wallet.address.to_string()),
    ]);

    let contract = factory
        .deploy_from_file(&ctx.hello_world_contract(), init, None)
        .await
        .unwrap();

    let response = contract.call("getHello", vec![]).await?;

    println!("{response:?}");

    Ok(())
}

#[test_context(TestContext)]
#[tokio::test]
async fn call_transition_with_single_string_param(ctx: &TestContext) -> Result<()> {
    let provider = ctx.provider();
    let factory = ContractFactory::new(provider.clone());

    let init = Init(vec![
        Value::new_from_str("_scilla_version", "Uint32", "0"),
        Value::new_from_str("owner", "ByStr20", &ctx.wallet.address.to_string()),
    ]);

    let contract = factory
        .deploy_from_file(&ctx.hello_world_contract(), init, None)
        .await
        .unwrap();

    let response = contract
        .call("setHello", vec![Value::new_from_str("msg", "String", "heellleeo")])
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
        Value::new_from_str("_scilla_version", "Uint32", "0"),
        Value::new_from_str("owner", "ByStr20", &ctx.wallet.address.to_string()),
    ]);

    let contract = factory
        .deploy_from_file(&ctx.hello_world_contract(), init, None)
        .await
        .unwrap();

    let contract = contract::HelloWorld::new(contract);

    let response = contract.get_hello().await?;

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
    let contract = contract::HelloWorld::deploy(provider, ctx.wallet.address.to_string()).await?;

    let response = contract.get_hello().await?;

    println!("{response:?}");

    let state = contract.get_state().await?;
    let hello = contract.welcome_msg().await?;
    assert_eq!(hello, state.welcome_msg);

    Ok(())
}

#[test_context(TestContext)]
#[tokio::test]
async fn call_transition_with_single_string_param_using_rust_binding(ctx: &TestContext) -> Result<()> {
    let provider = ctx.provider();
    let contract = contract::HelloWorld::deploy(provider, ctx.wallet.address.to_string()).await?;

    let response = contract.set_hello("heellleeo".to_string()).await?;

    println!("{response:?}");

    Ok(())
}
