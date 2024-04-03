use std::{env, str::FromStr, sync::Arc};

use test_context::AsyncTestContext;
use url::Url;
use zilliqa_rs::{middlewares::Middleware, providers::Provider, signers::LocalWallet};

pub struct TestContext {
    pub endpoint: Url,
    pub wallet: LocalWallet,
    pub chain_id: u16,
}

#[async_trait::async_trait]
impl AsyncTestContext for TestContext {
    async fn setup() -> Self {
        let endpoint = env::var("ZILLIQA_ENDPOINT")
            .unwrap_or("https://zilliqa-isolated-server.zilliqa.com".into())
            .parse()
            .unwrap();
        let wallet = LocalWallet::from_str(
            &env::var("TEST_WALLET").unwrap_or("d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba".to_string()),
        )
        .unwrap();

        let chain_id = env::var("ZILLIQA_CHAIN_ID").map(|s| s.parse().unwrap()).unwrap_or(222);
        Self {
            endpoint,
            wallet,
            chain_id,
        }
    }
}

impl TestContext {
    pub fn provider(&self) -> Arc<impl Middleware> {
        Arc::new(
            Provider::try_from(self.endpoint.clone())
                .unwrap()
                .with_chain_id(self.chain_id)
                .with_signer(self.wallet.clone()),
        )
    }
}
