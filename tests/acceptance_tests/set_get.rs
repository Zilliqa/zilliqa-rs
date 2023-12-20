use crate::common::TestContext;
use test_context::test_context;
use zilliqa_rs::{contract, crypto::ZilAddress};

#[test_context(TestContext)]
#[tokio::test]
async fn set_get_scenario(ctx: &TestContext) -> anyhow::Result<()> {
    let provider = ctx.provider();
    let contract = contract::SetGet::deploy(provider.clone()).await?;

    println!("{:?}", contract.address());
    contract.set_bool(true).call().await?;
    contract.set_option_bool(Some(true)).call().await?;
    contract.set_pair(("hey".to_string(), 123u32)).call().await?;
    contract.call_list(vec![]).call().await?;

    contract
        .call_list(vec!["988d047d9224412f76e61568f80016f8880ea898".parse::<ZilAddress>()?])
        .call()
        .await?;

    Ok(())
}
