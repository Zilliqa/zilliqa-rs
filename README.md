# Getting started
## Create a new Provider
### From a URL
```rust
use zilliqa_rs::providers::{Http, Provider};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let provider = Provider::<Http>::try_from("http://127.0.0.1:5555")?;
    Ok(())
}
```
### From a URL, with chain ID
```rust
use zilliqa_rs::providers::{Http, Provider};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let provider = Provider::<Http>::try_from("http://127.0.0.1:5555").unwrap().with_chain_id(1);
    Ok(())
}
```
### With new()
```rust
use zilliqa_rs::providers::{Http, Provider};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let chain_id = 1;
    let provider = Provider::new(Http::new(url::Url::parse("http://127.0.0.1:5555")?)?, chain_id);
    Ok(())
}
```

## Call RPC methods
```rust
use zilliqa_rs::providers::{Http, Provider};
use zilliqa_rs::middlewares::Middleware;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let provider = Provider::<Http>::try_from("http://127.0.0.1:5555")?.with_chain_id(1);
    let balance = provider.get_balance("0x381f4008505e940ad7681ec3468a719060caf796").await;
    Ok(())
}
```

## Send Transaction
The nonce can be omitted. Then the current nonce is fetched, incremented, and used as the next nonce.

```rust
use zilliqa_rs::providers::{Http, Provider};
use zilliqa_rs::transaction::TransactionBuilder;
use zilliqa_rs::signers::LocalWallet;
use zilliqa_rs::middlewares::Middleware;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const END_POINT: &str = "http://localhost:5555";

    let wallet = "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba".parse::<LocalWallet>()?;

    let provider = Provider::<Http>::try_from(END_POINT)?
        .with_chain_id(1)
        .with_signer(wallet.clone());

    let receiver = LocalWallet::create_random()?;
    let tx = TransactionBuilder::default()
        .to_address(receiver.address)
        .nonce(4)
        .amount(200u128 * 10u128.pow(12))
        .gas_price(2000000000u128)
        .gas_limit(50u64)
        .build();

    provider.send_transaction_without_confirm(tx).await?;
    Ok(())
}
```
