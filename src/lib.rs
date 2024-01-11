/*!
* This crate provides routines for interacting with [Zilliqa] network.
* Sending [JSON-RPC requests], sending transactions, deploying [scilla contracts],
* and interacting with them is easily possible using this crate.
*
* [Zilliqa]: https://www.zilliqa.com/
* [JSON-RPC requests]: https://dev.zilliqa.com/api/introduction/api-introduction/
* [scilla contracts]: https://scilla-lang.org/
*
* Here are some examples to get you started working with this crate.
*
* # Sending JSON-RPC requests
* The first step to do anything in zilliqa-rs is to create a `Provider`.
* Here we create an HTTP provider and call `GetBalance` JSON-RPC request.
* ```rust
* use zilliqa_rs::providers::{Http, Provider};
* use zilliqa_rs::middlewares::Middleware;
*
* #[tokio::main]
* async fn main() -> anyhow::Result<()> {
*     let provider = Provider::<Http>::try_from("http://127.0.0.1:5555")?.with_chain_id(222);
*     let balance = provider.get_balance("0x381f4008505e940ad7681ec3468a719060caf796").await;
*     Ok(())
* }
* ```
*/

pub mod contract;
pub mod core;
pub mod crypto;
pub mod error;
pub mod middlewares;
pub mod providers;
pub mod signers;
pub mod transaction;
pub mod util;

pub use error::Error;

/// Run them with `cargo test --doc`
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
