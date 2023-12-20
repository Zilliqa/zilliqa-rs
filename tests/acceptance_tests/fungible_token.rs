use crate::common::TestContext;
use test_context::test_context;
use zilliqa_rs::{contract, signers::LocalWallet};

#[test_context(TestContext)]
#[tokio::test]
async fn fungible_token_scenario(ctx: &TestContext) -> anyhow::Result<()> {
    let provider = ctx.provider();
    let contract = contract::FungibleToken::deploy(
        provider.clone(),
        ctx.wallet.address.clone(),
        "Sindal Token".to_string(),
        "SDT".to_string(),
        2,
        1000,
    )
    .await?;

    println!("{}", contract.address());

    // Check the general token info
    assert_eq!(contract.name().await?, "Sindal Token");
    assert_eq!(contract.symbol().await?, "SDT");
    assert_eq!(contract.init_supply().await?, 1000);

    // Should have 1000 as contract owner's balance
    let balances = contract.balances().await?;
    assert_eq!(balances[&ctx.wallet.address.to_string().to_lowercase()], "1000");

    // Should be possible to increase allowance by contract owner
    let alice = LocalWallet::create_random()?;
    contract.increase_allowance(alice.address.clone(), 100).call().await?;
    assert_eq!(
        contract.allowances().await?[&ctx.wallet.address.to_string().to_lowercase()][&alice.address.to_string().to_lowercase()],
        "100"
    );

    // Should be possible to decrease allowance by contract owner
    contract.decrease_allowance(alice.address.clone(), 10).call().await?;
    assert_eq!(
        contract.allowances().await?[&ctx.wallet.address.to_string().to_lowercase()][&alice.address.to_string().to_lowercase()],
        "90"
    );

    Ok(())
}
