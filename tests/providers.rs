use anyhow::Result;
use claim::assert_ok;
use url::Url;
use zilliqa_rs::{
    middlewares::Middleware,
    providers::{Http, Provider},
};

#[tokio::test]
async fn http_provider_should_work_fine_with_url_parse() {
    let provider = Provider::<Http>::try_from("http://127.0.0.1:5555");
    assert_ok!(&provider);

    let provider = provider.unwrap();

    println!(
        "{:?}",
        provider.get_balance("0x381f4008505e940ad7681ec3468a719060caf796").await
    );
}

#[tokio::test]
async fn http_provider_should_work_fine_with_new() -> Result<()> {
    let chain_id = 1;
    let provider = Provider::new(Http::new(Url::parse("http://127.0.0.1:5555")?)?, chain_id);

    println!(
        "{:?}",
        provider.get_balance("0x381f4008505e940ad7681ec3468a719060caf796").await
    );

    Ok(())
}
