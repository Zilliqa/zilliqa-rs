use std::sync::Arc;

use anyhow::Result;
use claim::assert_gt;
use zilliqa_rs::{
    middlewares::Middleware,
    providers::{CreateTransactionResponse, Http, Provider},
    signers::LocalWallet,
    transaction::{Transaction, TransactionBuilder, Version},
    util::parse_zil,
    Error,
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
        .amount(parse_zil("2")?)
        .gas_price(parse_zil("0.002")?)
        .gas_limit(50u64)
        .build();

    provider
        .send_transaction_without_confirm::<CreateTransactionResponse>(tx)
        .await?;

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
        .amount(parse_zil("2")?)
        .gas_price(parse_zil("0.002")?)
        .gas_limit(50u64)
        .build();

    let res: Result<CreateTransactionResponse, _> = provider.send_transaction_without_confirm(tx).await;
    assert!(matches!(
        res,
        Err(Error::InvalidVersionIsSetForTransaction(v)
        ) if v == Version::new(0)
    ));

    Ok(())
}

#[tokio::test]
async fn send_zil_using_pay_function() -> Result<()> {
    const END_POINT: &str = "http://localhost:5555";

    let wallet = "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba".parse::<LocalWallet>()?;
    let provider = Provider::<Http>::try_from(END_POINT)?
        .with_chain_id(1)
        .with_signer(wallet.clone());

    let sender_balance = provider.get_balance(&wallet.address).await?;

    assert_gt!(sender_balance.balance, 200u128);

    let receiver = LocalWallet::create_random()?;
    let amount = parse_zil("2")?;

    let tx = TransactionBuilder::default().pay(amount, receiver.address.clone()).build();
    provider.send_transaction_without_confirm(tx).await?;

    let res = provider.get_balance(&receiver.address).await?;

    assert_gt!(res.balance, 200u128);

    Ok(())
}

#[ignore]
#[tokio::test]
async fn get_transaction_receipt() -> Result<()> {
    const END_POINT: &str = "https://api.devnet.zilliqa.com";

    let wallet = "0x8ce73c46c1b8d09171319cf1498e538bbd151a4b65d6688cccdee1473d626c49".parse::<LocalWallet>()?;
    let provider = Provider::<Http>::try_from(END_POINT)?
        .with_chain_id(617)
        .with_signer(wallet.clone());

    let provider = Arc::new(provider);
    let sender_balance = provider.get_balance(&wallet.address).await?;
    println!("{sender_balance:?}");

    let receiver = LocalWallet::create_random()?;
    let amount = parse_zil("2")?;

    let tx = TransactionBuilder::default().pay(amount, receiver.address.clone()).build();
    let res = provider
        .send_transaction_without_confirm::<CreateTransactionResponse>(tx)
        .await?;

    println!("{res:?}");
    let tx = Transaction::new(res.tran_id, provider.clone());
    let receipt = tx.receipt().await?;
    println!("{:?}", receipt.borrow());

    let sender_balance = provider.get_balance(&receiver.address).await?;
    println!("{sender_balance:?}");

    Ok(())
}
