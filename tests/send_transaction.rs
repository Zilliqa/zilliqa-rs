use anyhow::Result;
use claim::assert_gt;
use zilliqa_rs::{
    middlewares::{Middleware, MiddlewareError},
    providers::{Http, Provider, ProviderError},
    signers::LocalWallet,
    transaction::{TransactionBuilder, Version},
};

#[tokio::test]
async fn send_transaction() -> Result<()> {
    const END_POINT: &str = "http://localhost:5555";

    let wallet = "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba".parse::<LocalWallet>()?;

    let provider = Provider::<Http>::try_from(END_POINT)?
        .with_chain_id(1)
        .with_signer(wallet.clone());

    let sender_balance = provider.get_balance(&wallet.address).await?;

    assert_gt!(sender_balance.balance, 200u128);

    let receiver = LocalWallet::create_random()?;
    let tx = TransactionBuilder::default()
        .to_address(receiver.address.clone())
        .amount(200u128 * 10u128.pow(12))
        .gas_price(2000000000u128)
        .gas_limit(50u64)
        .build();

    provider.send_transaction(tx).await?;

    let res = provider.get_balance(&receiver.address).await?;

    assert_gt!(res.balance, 200u128);

    Ok(())
}

#[tokio::test]
async fn if_version_is_not_set_create_transaction_should_return_error() -> Result<()> {
    const END_POINT: &str = "http://localhost:5555";

    let wallet = "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba".parse::<LocalWallet>()?;

    // Chain ID is not set, so version
    let provider = Provider::<Http>::try_from(END_POINT)?.with_signer(wallet.clone());

    let receiver = LocalWallet::create_random()?;
    let tx = TransactionBuilder::default()
        .to_address(receiver.address)
        .amount(200u128 * 10u128.pow(12))
        .gas_price(2000000000u128)
        .gas_limit(50u64)
        .build();

    let res = provider.send_transaction(tx).await;
    assert!(matches!(
        res,
        Err(MiddlewareError::ProviderError(
            ProviderError::InvalidVersionIsSetForTransaction(v)
        )) if v == Version::new(0)
    ));

    Ok(())
}
