use zilliqa_rs::{
    account::{Account, Transaction},
    crypto::util::to_checksum_address,
    zilliqa::Zilliqa,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let zilliqa = Zilliqa::new("http://127.0.0.1:5555").unwrap();

    let tx = Transaction {
        version: 65537,
        nonce: 11,
        to_addr: to_checksum_address("0xf6dad9e193fa2959a849b81caf9cb6ecde466771")?,
        amount: 200u128 * 10u128.pow(12),
        pub_key: None,
        gas_price: 2000000000u128,
        gas_limit: 50u64,
        code: "".to_string(),
        data: "".to_string(),
        signature: None,
    };

    let account = Account::new("d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba")?;
    let tx = account.sign_transaction(tx);

    let res = zilliqa.blockchain.send_transaction(tx).await.unwrap();
    println!("{:?}", res);

    Ok(())
}
