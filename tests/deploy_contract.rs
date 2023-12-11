use std::{env, path::PathBuf, sync::Arc};
use test_context::{test_context, AsyncTestContext};

use anyhow::Result;
use url::Url;
use zilliqa_rs::{
    contract::{self, ContractFactory, Init, Value},
    middlewares::Middleware,
    providers::Provider,
    signers::LocalWallet,
};

struct DeploymentTestContext {
    endpoint: Url,
    wallet: LocalWallet,
    chain_id: u16,
    timestamp_contract: PathBuf,
    hello_world_contract: PathBuf,
}

#[async_trait::async_trait]
impl AsyncTestContext for DeploymentTestContext {
    async fn setup() -> Self {
        let endpoint = env::var("ZILLIQA_ENDPOINT")
            .unwrap_or("http://localhost:5555".into())
            .parse()
            .unwrap();
        let wallet = LocalWallet::new(
            &env::var("TEST_WALLET").unwrap_or("d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba".to_string()),
        )
        .unwrap();

        let chain_id = env::var("ZILLIQA_CHAIN_ID").map(|s| s.parse().unwrap()).unwrap_or(1);
        Self {
            endpoint,
            wallet,
            chain_id,
            timestamp_contract: PathBuf::from("tests/contracts/Timestamp.scilla"),
            hello_world_contract: PathBuf::from("tests/contracts/HelloWorld.scilla"),
        }
    }
}

impl DeploymentTestContext {
    fn provider(&self) -> Arc<impl Middleware> {
        Arc::new(
            Provider::try_from(self.endpoint.clone())
                .unwrap()
                .with_chain_id(self.chain_id)
                .with_signer(self.wallet.clone()),
        )
    }
}

#[test_context(DeploymentTestContext)]
#[tokio::test]
async fn deploy_contract_without_constructor_parameter(ctx: &DeploymentTestContext) -> Result<()> {
    let provider = ctx.provider();

    let factory = ContractFactory::new(provider.clone());

    let init = Init(vec![Value::new_from_str("_scilla_version", "Uint32", "0")]);

    let contract = factory.deploy_from_file(&ctx.timestamp_contract, init, None).await.unwrap();

    println!("addr: {:?}", contract);
    Ok(())
}

#[test_context(DeploymentTestContext)]
#[tokio::test]
async fn deploy_contract_with_constructor_parameter(ctx: &DeploymentTestContext) -> Result<()> {
    let provider = ctx.provider();
    let factory = ContractFactory::new(provider);

    let init = Init(vec![
        Value::new_from_str("_scilla_version", "Uint32", "0"),
        Value::new_from_str("owner", "ByStr20", &ctx.wallet.address.to_string()),
    ]);

    let contract = factory.deploy_from_file(&ctx.hello_world_contract, init, None).await.unwrap();

    println!("addr: {:?}", contract);
    Ok(())
}

#[test_context(DeploymentTestContext)]
#[tokio::test]
async fn deploy_from_string(ctx: &DeploymentTestContext) -> Result<()> {
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

#[test_context(DeploymentTestContext)]
#[tokio::test]
async fn call_a_param_less_transition(ctx: &DeploymentTestContext) -> Result<()> {
    let provider = ctx.provider();
    let factory = ContractFactory::new(provider.clone());

    let init = Init(vec![
        Value::new_from_str("_scilla_version", "Uint32", "0"),
        Value::new_from_str("owner", "ByStr20", &ctx.wallet.address.to_string()),
    ]);

    let contract = factory.deploy_from_file(&ctx.hello_world_contract, init, None).await.unwrap();

    let response = contract.call("getHello", vec![]).await?;

    println!("{response:?}");

    Ok(())
}

#[test_context(DeploymentTestContext)]
#[tokio::test]
async fn call_transition_with_single_string_param(ctx: &DeploymentTestContext) -> Result<()> {
    let provider = ctx.provider();
    let factory = ContractFactory::new(provider.clone());

    let init = Init(vec![
        Value::new_from_str("_scilla_version", "Uint32", "0"),
        Value::new_from_str("owner", "ByStr20", &ctx.wallet.address.to_string()),
    ]);

    let contract = factory.deploy_from_file(&ctx.hello_world_contract, init, None).await.unwrap();

    let response = contract
        .call("setHello", vec![Value::new_from_str("msg", "String", "heellleeo")])
        .await?;

    println!("{response:?}");

    Ok(())
}

#[test_context(DeploymentTestContext)]
#[tokio::test]
async fn call_a_param_less_transition_though_the_rust_binding(ctx: &DeploymentTestContext) -> Result<()> {
    let provider = ctx.provider();
    let factory = ContractFactory::new(provider.clone());

    let init = Init(vec![
        Value::new_from_str("_scilla_version", "Uint32", "0"),
        Value::new_from_str("owner", "ByStr20", &ctx.wallet.address.to_string()),
    ]);

    let contract = factory.deploy_from_file(&ctx.hello_world_contract, init, None).await.unwrap();

    let contract = contract::HelloWorld::new(contract);

    let response = contract.get_hello().await?;

    println!("{response:?}");

    Ok(())
}

#[test_context(DeploymentTestContext)]
#[tokio::test]
async fn deploy_a_paramless_contract_through_the_rust_binding(ctx: &DeploymentTestContext) -> Result<()> {
    let provider = ctx.provider();
    let _contract = contract::Timestamp::deploy(provider).await?;
    Ok(())
}

#[test_context(DeploymentTestContext)]
#[tokio::test]
async fn deploy_a_one_param_contract_through_the_rust_binding(ctx: &DeploymentTestContext) -> Result<()> {
    let provider = ctx.provider();
    let contract = contract::HelloWorld::deploy(provider, ctx.wallet.address.to_string()).await?;

    let response = contract.get_hello().await?;

    println!("{response:?}");

    let state = contract.get_state().await?;
    let hello = contract.welcome_msg().await?;
    assert_eq!(hello, state.welcome_msg);

    Ok(())
}

#[test_context(DeploymentTestContext)]
#[tokio::test]
async fn call_transition_with_single_string_param_using_rust_binding(ctx: &DeploymentTestContext) -> Result<()> {
    let provider = ctx.provider();
    let contract = contract::HelloWorld::deploy(provider, ctx.wallet.address.to_string()).await?;

    let response = contract.set_hello("heellleeo".to_string()).await?;

    println!("{response:?}");

    Ok(())
}
