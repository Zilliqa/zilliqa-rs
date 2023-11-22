use zilliqa_rs::{
    middlewares::Middleware,
    providers::{Http, Provider},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let provider = Provider::<Http>::try_from("http://127.0.0.1:5555")?;

    println!(
        "{:?}",
        provider.get_balance("0x381f4008505e940ad7681ec3468a719060caf796").await
    );

    Ok(())
}
