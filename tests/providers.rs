use zilliqa_rs::providers::{Http, Provider};

#[tokio::test]
async fn http_provider_should_work_fine() {
    let provider = Provider::<Http>::try_from("http://127.0.0.1:5555").unwrap();

    println!(
        "{:?}",
        provider.get_balance("0x381f4008505e940ad7681ec3468a719060caf796").await
    );
}
