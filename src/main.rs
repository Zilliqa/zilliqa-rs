use zilliqa_rs::zilliqa::Zilliqa;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let zilliqa = Zilliqa::new("http://127.0.0.1:5555").unwrap();
    let balance = zilliqa
        .blockchain
        .get_balance("zil18q05qzzst62q44mgrmp5dzn3jpsv4aukxredu2")
        .await;

    println!("{:?}", balance);
    Ok(())
}
