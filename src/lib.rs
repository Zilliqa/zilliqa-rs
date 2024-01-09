pub mod contract;
pub mod core;
pub mod crypto;
pub mod error;
pub mod middlewares;
pub mod proto;
pub mod providers;
pub mod signers;
pub mod transaction;
pub mod util;

pub use error::Error;

/// Run them with `cargo test --doc`
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
