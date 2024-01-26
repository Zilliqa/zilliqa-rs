use crate::common::TestContext;
use test_context::test_context;
use zilliqa_rs::{
    contract::{self, ScillaValue},
    middlewares::Middleware,
};

#[test_context(TestContext)]
#[tokio::test]
async fn chainid_scenario(ctx: &TestContext) -> anyhow::Result<()> {
    let provider = ctx.provider();
    let contract = contract::ChainId::deploy(provider.clone()).await?;

    let tx = contract.event_chain_id().call().await?;
    let event = tx.receipt.event_log("ChainID").unwrap();
    assert!(matches!(&event.params[0].value, ScillaValue::Primitive(x) if x == &provider.chainid().to_string()));

    Ok(())
}
