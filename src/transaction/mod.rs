/*!
Send transaction to zilliqa network

This module is used to send a transaction to zilliqa network. To compose a transaction,
It's easier to use [TransactionBuilder].

*/

pub mod builder;
pub mod version;

use std::cell::Cell;

pub use builder::*;
pub use version::*;

use crate::{
    core::GetTransactionResponse,
    middlewares::Middleware,
    providers::{JsonRpcClient, Provider},
    Error,
};

#[derive(Debug)]
pub struct Transaction<'a, T: JsonRpcClient> {
    pub id: String,
    client: &'a Provider<T>,
    status: Cell<TxStatus>,
}

#[derive(Debug, Clone, Copy)]
pub enum TxStatus {
    Initialized,
    Pending,
    Confirmed,
    Rejected,
}

impl<'a, T: JsonRpcClient> Transaction<'a, T> {
    pub fn new(id: String, client: &'a Provider<T>) -> Self {
        Self {
            id,
            client,
            status: Cell::new(TxStatus::Initialized),
        }
    }

    /// Tries to confirm a transaction.
    pub async fn confirm(&self) -> Result<GetTransactionResponse, Error> {
        self.try_confirm(tokio::time::Duration::from_secs(10), 33).await
    }

    /// The `try_confirm` function attempts to confirm a transaction by making repeated requests to a
    /// client, with a specified interval and maximum number of attempts, and returns the result.
    ///
    /// Arguments:
    ///
    /// * `interval`: The `interval` parameter is the duration of time to wait between each attempt to
    /// confirm the transaction. It is of type `tokio::time::Duration`, which represents a duration of time
    /// in the Tokio runtime.
    /// * `max_attempt`: The `max_attempt` parameter is the maximum number of attempts to confirm the
    /// transaction.
    pub async fn try_confirm(&self, interval: tokio::time::Duration, max_attempt: u32) -> Result<GetTransactionResponse, Error> {
        self.status.set(TxStatus::Pending);
        for _ in 0..max_attempt {
            let res = match self.client.get_transaction(&self.id).await {
                Ok(res) => res,
                Err(_) => {
                    tokio::time::sleep(interval).await;
                    continue;
                }
            };
            self.status.set(if res.receipt.success {
                TxStatus::Confirmed
            } else {
                TxStatus::Rejected
            });

            return Ok(res);
        }

        Err(Error::UnableToConfirmTransaction(max_attempt))
    }
}
