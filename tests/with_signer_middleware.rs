use zilliqa_rs::{
    account::{Transaction, TransactionBuilder},
    middlewares::Middleware,
    providers::{Http, Provider},
    signers::LocalWallet,
};

// TODO: Make it a real test
#[tokio::test]
async fn test_with_signer_middleware() -> Result<(), Box<dyn std::error::Error>> {
    const END_POINT: &str = "https://api.zilliqa.com";

    let wallet = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7".parse::<LocalWallet>()?;
    let provider = Provider::<Http>::try_from(END_POINT).unwrap().with_signer(wallet);

    let tx: Transaction = TransactionBuilder::default()
        .to_address(&"0xf6dad9e193fa2959a849b81caf9cb6ecde466771")
        .amount(200u128 * 10u128.pow(12))
        .gas_price(2000000000u128)
        .gas_limit(50u64)
        .into();

    let signature = provider.sign_transaction(&tx).unwrap();
    println!("{:?}", signature);

    Ok(())
}
