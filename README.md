# Getting started
## Create a new Rust app
The very first step is to create a binary rust project.

```bash
cargo new zilliqa-rs-demo
```

then we need to add zilliqa-rs and tokio to the project's dependencies:

```bash
cargo add zilliqa-rs tokio
```

## Call a simple JSON-RPC API
### Run the isolated-server using docker
Here we run an isolated server using docker to use it as the target network, but you can use any zilliqa network you want.
```bash
docker run -d -p 5555:5555 --name iso-server zilliqa-isolated-server:latest
```

### Call GetBalance
First, we need to create a provider. In the first line of the main, we create an HTTP provider. We use the URL of the isolated server we ran in the previous step. The chain ID of this network is 222. 
Then we can call the `get_balance` function of the provider, passing the address of the account we want its balance.

```rust
use std::error::Error;

use zilliqa_rs::middlewares::Middleware;
use zilliqa_rs::providers::{Http, Provider};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let provider = Provider::<Http>::try_from("http://127.0.0.1:5555")?.with_chain_id(222);
    let balance = provider
        .get_balance("0x381f4008505e940ad7681ec3468a719060caf796")
        .await;

    println!("{balance:?}");
    Ok(())
}
```

## Working with contracts
### Technical notes
One of the coolest features of zilliqa-rs is generating rust code for your scilla contracts during build time. It means if your contract has a transition like `transfer`, you can call it the same as a normal rust function. If it has a parameter of an address, you must pass an address to this function. And this means all of the beauties of type-checking of rust come to working with scilla contracts.

### Generating rust code from scilla contracts
We want to deploy a simple contract named `HelloWorld` and call its `setHello` transition. First, we need to create a folder next to `src`. Let's call it `contracts`. Then we move [HelloWorld.scilla](./tests/contracts/HelloWorld.scilla) to this folder. To let zilliqa-rs scilla-to-rust code generation know about the contracts path, we need to export `CONTRACTS_PATH` environment variable. The simplest way is to create `.cargo/config.toml` file and change it like:

```toml
[env]
CONTRACTS_PATH = {value = "contracts", relative = true}
```
setting `relative` to `true` is crucial. Otherwise, your scilla contracts won't be transpiled to rust. Now, if you build the project using `cargo build`, your HelloWorld.scilla gets converted to rust under the hood.

The generated code is something like this:

```rust
impl<T: Middleware> HelloWorld<T> {
    pub async fn deploy(client: Arc<T> , owner: ZilAddress) -> Result<Self, Error> {
    }

    pub fn address(&self) -> &ZilAddress  {
    }
    
    pub fn set_hello(&self , msg: String) -> RefMut<'_, transition_call::TransitionCall<T>> {
    }

    pub fn get_hello(&self ) -> RefMut<'_, transition_call::TransitionCall<T>> {
    }

    pub async fn welcome_msg(&self) -> Result<String, Error> {
    }

    pub async fn owner(&self) -> Result<ZilAddress, Error> {
    }
}
```
* The `deploy` deploys the contract to the network. Because HelloWorld.scilla contract accepts an address, `owner`, as a deployment parameter, the `deploy` function needs that too. It means you can't deploy it without providing a valid address.
* The `address` function returns the address of the deployed contract.
* `set_hello` corresponds to `setHello` transition in the contract. Again, because the transition accepts a string parameter, the `set_hello` function does too. 
* `get_hello` corresponds to the `getHello` transition.
* The contract has a field named, `welcome_msg`, to get the value of this field, the `welcome_msg` function should be called.
* The contract has an immutable state named, `owner` and we passed the value during deployment. To get the value of the owner, we need to call `owner`

### Provider with a signer
To deploy the contract, we need to change the provider. The provider we had so far, didn't have a signer. That was because we didn't want to send transactions. But to deploy the contract we do need to have a provider with a signer:

```rust
    let wallet = "0xe53d1c3edaffc7a7bab5418eb836cf75819a82872b4a1a0f1c7fcf5c3e020b89"
        .parse::<LocalWallet>()?;

    let provider = Provider::<Http>::try_from("http://127.0.0.1:5555")?
        .with_chain_id(222)
        .with_signer(wallet.clone());
```
Here, we create a new wallet from a private key and a provider with that signer. This provider now can be used to send transactions.

### Contract Deployment
Now it's time to deploy the contract:
```rust
    let contract = HelloWorld::deploy(provider.into(), wallet.address).await?;
    println!("Contract address: {:?}", contract.address());
```
The first parameter to `deploy` is the provider. The rest depends on the contract and how many immutable states it has. Here in HelloWorld.scilla we only have `owner`, so we just pass an address. It's type-safe, it means you can't pass an integer or even a raw string to `deploy` function as `owner`.

Run the code: 

```bash
cargo run

Ok(BalanceResponse { nonce: 138, balance: 899999994124734000000000 })
Contract address: ZilAddress("0xC50C93831F6eAB4e4F011076dca6e887288cc872")
```

### Getting contract states
Our contract has `owner`, an immutable state, and `welcome_msg`, a mutable one. We can get these states by calling the corresponding functions:
```rust
    println!("Contract owner: {:?}", contract.owner().await?);
    println!("Welcome msg: {}", contract.welcome_msg().await?);
```

### Calling a transition
Our contract has a `setHello` transition. Calling this transition is not harder than calling a rust function:

```rust
    contract.set_hello("Salaam".to_string()).call().await?;
```
Pay attention, here we need to call `call` too. That's because everything you do before `call` is like configuring the transition call. For example, you can set the amount of ZIL you want to pass to a transition before calling `call` function:
```rust
    contract.transfer(receiver).amount(parse_zil("0.1")).call().await?;
```

OK, now if you get and print `welcome_msg` it should have the new value:
```rust
    println!("Welcome msg: {}", contract.welcome_msg().await?);
```
The final main code is:
```rust
use std::error::Error;

use zilliqa_rs::{
    contract::HelloWorld,
    middlewares::Middleware,
    providers::{Http, Provider},
    signers::LocalWallet,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let wallet = "0xe53d1c3edaffc7a7bab5418eb836cf75819a82872b4a1a0f1c7fcf5c3e020b89"
        .parse::<LocalWallet>()?;

    let provider = Provider::<Http>::try_from("http://127.0.0.1:5555")?
        .with_chain_id(222)
        .with_signer(wallet.clone());

    let balance = provider
        .get_balance("0x381f4008505e940ad7681ec3468a719060caf796")
        .await;

    println!("{balance:?}");

    let contract = HelloWorld::deploy(provider.into(), wallet.address).await?;
    println!("Contract address: {:?}", contract.address());

    println!("Contract owner: {:?}", contract.owner().await?);
    println!("Welcome msg: {}", contract.welcome_msg().await?);

    contract.set_hello("Salaam".to_string()).call().await?;
    println!("Welcome msg: {}", contract.welcome_msg().await?);
    Ok(())
}
```

Let's run the code:

```bash
cargo run

Ok(BalanceResponse { nonce: 138, balance: 899999994124734000000000 })
Contract address: ZilAddress("0xB84De4A67E1640D9259c502AAb6751678B593185")
Contract owner: ZilAddress("0xd90f2e538CE0Df89c8273CAd3b63ec44a3c4ed82")
Welcome msg: Hello world!
Welcome msg: Salaam
```

## Send Transaction
The nonce can be omitted. Then the current nonce is fetched, incremented, and used as the next nonce.

```rust
use zilliqa_rs::providers::{Http, Provider};
use zilliqa_rs::core::CreateTransactionResponse;
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
use zilliqa_rs::providers::{Http, Provider};
use zilliqa_rs::core::CreateTransactionResponse;
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