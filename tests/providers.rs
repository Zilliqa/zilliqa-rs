use claim::assert_ok;
use zilliqa_rs::{
    middlewares::Middleware,
    providers::{Http, Provider},
};

// TODO: Make it a real test
#[tokio::test]
async fn http_provider_should_work_fine() {
    let provider = Provider::<Http>::try_from("http://127.0.0.1:5555");
    assert_ok!(&provider);

    let provider = provider.unwrap();

    println!(
        "{:?}",
        provider.get_balance("0x381f4008505e940ad7681ec3468a719060caf796").await
    );
}
