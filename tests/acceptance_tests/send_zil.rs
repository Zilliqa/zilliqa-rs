use crate::common::TestContext;
use test_context::test_context;
use zilliqa_rs::{contract, middlewares::Middleware, signers::LocalWallet, util::parse_zil};

#[test_context(TestContext)]
#[tokio::test]
async fn send_zil_scenario(ctx: &TestContext) -> anyhow::Result<()> {
    let provider = ctx.provider();
    let contract = contract::SendZil::deploy(provider.clone()).await?;

    // Send ZIL to contract
    contract.accept_zil().amount(parse_zil("0.5")?).call().await?;
    assert_eq!(provider.get_balance(contract.address()).await?.balance, parse_zil("0.5")?);

    // Should have untouched balance because accept is NOT called in the dontAcceptZil transition
    contract.dont_accept_zil().amount(parse_zil("1.0")?).call().await?;
    assert_eq!(
        provider.get_balance(contract.address()).await?.balance,
        parse_zil("0.5")?,
        "Balance should not have changed"
    );

    // Should be possible to fund a user
    let wallet = LocalWallet::create_random()?;
    contract
        .fund_user(wallet.address.to_string(), parse_zil("0.1")?)
        .call()
        .await?;
    assert_eq!(
        provider.get_balance(&wallet.address).await?.balance,
        parse_zil("0.1")?,
        "User should have received funds"
    );
    Ok(())
}
