use zilliqa_rs::zilliqa::Zilliqa;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let zilliqa = Zilliqa::new("http://127.0.0.1:5555").unwrap();
    let balance = zilliqa
        .blockchain
        .get_balance("0x381f4008505e940ad7681ec3468a719060caf796")
        .await;

    println!("{:?}", balance);
    Ok(())
}
