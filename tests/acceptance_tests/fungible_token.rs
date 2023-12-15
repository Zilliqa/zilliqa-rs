use crate::common::TestContext;
use test_context::test_context;
use zilliqa_rs::contract;

#[test_context(TestContext)]
#[tokio::test]
async fn fungible_token_scenario(ctx: &TestContext) -> anyhow::Result<()> {
    let provider = ctx.provider();
    let contract = contract::FungibleToken::deploy(
        provider.clone(),
        ctx.wallet.address.to_string(),
        "Sindal Token".to_string(),
        "SDT".to_string(),
        2,
        1000,
    )
    .await?;

    assert_eq!(contract.name().await?, "Sindal Token");
    assert_eq!(contract.symbol().await?, "SDT");
    assert_eq!(contract.init_supply().await?, 1000);

    Ok(())
}
