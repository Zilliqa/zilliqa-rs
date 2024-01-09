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
### With chain ID
```rust
use zilliqa_rs::providers::{Http, Provider};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let provider = Provider::<Http>::try_from("http://127.0.0.1:5555").unwrap().with_chain_id(1);
    Ok(())
}
```

### With a signer
If a provider has a designated signer, all transactions requiring signing will be signed using the designated signer before being sent to the endpoint.

```rust
use zilliqa_rs::providers::{Http, Provider};
use zilliqa_rs::signers::LocalWallet;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let wallet = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7".parse::<LocalWallet>()?;
    let provider = Provider::<Http>::try_from("http://127.0.0.1").unwrap().with_signer(wallet);
    Ok(())
}
```

## Call RPC methods
```rust
use zilliqa_rs::providers::{Http, Provider};
use zilliqa_rs::middlewares::Middleware;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let provider = Provider::<Http>::try_from("http://127.0.0.1:5555")?.with_chain_id(222);
    let balance = provider.get_balance("0x381f4008505e940ad7681ec3468a719060caf796").await;
    Ok(())
}
```

## Send Transaction
The nonce can be omitted. Then the current nonce is fetched, incremented, and used as the next nonce.

```rust
use zilliqa_rs::providers::{CreateTransactionResponse, Http, Provider};
use zilliqa_rs::transaction::TransactionBuilder;
use zilliqa_rs::signers::LocalWallet;
use zilliqa_rs::middlewares::Middleware;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const END_POINT: &str = "http://localhost:5555";

    let wallet = "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba".parse::<LocalWallet>()?;

    let provider = Provider::<Http>::try_from(END_POINT)?
        .with_chain_id(222)
        .with_signer(wallet.clone());

    let receiver = LocalWallet::create_random()?;
    let tx = TransactionBuilder::default()
        .to_address(receiver.address)
        .amount(200u128 * 10u128.pow(12))
        .gas_price(2000000000u128)
        .gas_limit(50u64)
        .build();

    provider.send_transaction_without_confirm::<CreateTransactionResponse>(tx).await?;
    Ok(())
}
```

### Use pay() function
TransactionBuilder has an auxiliary function named `pay` to simplify payment transaction creation:

```rust
use zilliqa_rs::providers::{CreateTransactionResponse, Http, Provider};
use zilliqa_rs::transaction::TransactionBuilder;
use zilliqa_rs::signers::LocalWallet;
use zilliqa_rs::middlewares::Middleware;
use zilliqa_rs::util::parse_zil;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const END_POINT: &str = "http://localhost:5555";

    let wallet = "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba".parse::<LocalWallet>()?;
    let provider = Provider::<Http>::try_from(END_POINT)?
        .with_chain_id(222)
        .with_signer(wallet.clone());

    let receiver = LocalWallet::create_random()?;
    let amount = parse_zil("0.2")?;

    let tx = TransactionBuilder::default().pay(amount, receiver.address.clone()).build();
    provider.send_transaction_without_confirm::<CreateTransactionResponse>(tx).await?;

    Ok(())
}
```

## Contracts

### Deployment using rust binding
You can place your contracts in `contracts` folder or override the default path by exporting `CONTRACTS_PATH` variable. If you have docker installed, all a rust binding will be generated for each contract during build.

So if you have a contract like `Timestamp.scilla` in the contracts folder containing:

```scilla
scilla_version 0

transition EventTimestamp (bnum: BNum)
ts <-& TIMESTAMP(bnum);
e = { _eventname : "TS"; timestamp : ts };
event e
end
```

You can deploy it `contract::Timestamp::deploy`:

```rust
use std::sync::Arc;

use zilliqa_rs::{
    contract,
    providers::{Http, Provider},
    signers::LocalWallet,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const END_POINT: &str = "http://localhost:5555";

    let wallet = "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba".parse::<LocalWallet>()?;

    let provider = Provider::<Http>::try_from(END_POINT)?
        .with_chain_id(222)
        .with_signer(wallet.clone());

    let contract = contract::Timestamp::deploy(Arc::new(provider)).await?;

    Ok(())
}

```
If the contract needs some initial parameters to deploy, You must pass them to `deploy` function, otherwise you won't be able to compile the code.

Instead of using rust binding, it's possible to use `deploy_from_file` or `deploy_str` functions from `ContractFactory` to deploy a contract manually. Take a look at `deploy_contract_without_constructor_parameter` and `deploy_contract_with_constructor_parameter` and `deploy_from_string` tests in the [deployment tests](./tests/deploy_contract.rs)


### Calling a transition

The [HelloWorld](./tests/contracts/HelloWorld.scilla) contract has a `setHello` transition. It can be called in rust like:
```rust
use std::sync::Arc;

use zilliqa_rs::{
    contract,
    core::BNum,
    providers::{Http, Provider},
    signers::LocalWallet,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const END_POINT: &str = "http://localhost:5555";

    let wallet = "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba".parse::<LocalWallet>()?;

    let provider = Provider::<Http>::try_from(END_POINT)?
        .with_chain_id(222)
        .with_signer(wallet.clone());

    let contract = contract::HelloWorld::deploy(Arc::new(provider), wallet.address.clone()).await?;
    contract.set_hello("Salaam".to_string()).call().await?;

    Ok(())
}
```
If a transition needs some parameters, like here, You must pass them too, otherwise you won't be able to compile the code.

### Calling a transaction with custom parameters for nonce, amount, etc.
It's possible to override default transaction parameters such as nonce and amount.
```rust
use zilliqa_rs::{contract, middlewares::Middleware, util::parse_zil, signers::LocalWallet, providers::{Http, Provider}};
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const END_POINT: &str = "http://localhost:5555";

    let wallet = "e53d1c3edaffc7a7bab5418eb836cf75819a82872b4a1a0f1c7fcf5c3e020b89".parse::<LocalWallet>()?;

    let provider = Arc::new(Provider::<Http>::try_from(END_POINT)?
        .with_chain_id(222)
        .with_signer(wallet.clone()));

    let contract = contract::SendZil::deploy(provider.clone()).await?;
    // Override the amount before sending the transaction.
    contract.accept_zil().amount(parse_zil("0.5")?).call().await?;
    assert_eq!(provider.get_balance(contract.address()).await?.balance, parse_zil("0.5")?);
    Ok(())
}
```

It's possible to call a transition without using rust binding. Take a look at `call_a_param_less_transition` and `call_transition_with_single_string_param` tests in the [deployment tests](./tests/deploy_contract.rs). 

### Getting the contract's state
Suppose we have a contract like this: 
```scilla
contract HelloWorld
(owner: ByStr20)

field welcome_msg : String = "Hello world!"
```
You can get the latest state of the contract by calling `welcome_msg` function:
```rust
use std::sync::Arc;

use zilliqa_rs::{
    contract,
    providers::{Http, Provider},
    signers::LocalWallet,
};
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const END_POINT: &str = "http://localhost:5555";

    let wallet = "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba".parse::<LocalWallet>()?;

    let provider = Provider::<Http>::try_from(END_POINT)?
        .with_chain_id(222)
        .with_signer(wallet.clone());

    let contract = contract::HelloWorld::deploy(Arc::new(provider), wallet.address.clone()).await?;

    let hello = contract.welcome_msg().await?;
    assert_eq!(hello, "Hello world!".to_string());

    contract.set_hello("Salaam".to_string()).call().await?;
    let hello = contract.welcome_msg().await?;
    assert_eq!(hello, "Salaam".to_string());
    Ok(())
}
```
As you can see in the above code snippet, you can get individual states like `welcome_msg` through a function with the same name, `welcome_msg()`.