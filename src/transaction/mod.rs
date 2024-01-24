/*!
Send transaction to zilliqa network

This module is used to send a transaction to zilliqa network. To compose a transaction,
It's easier to use [TransactionBuilder].

*/

pub mod builder;

use std::{cell::Cell, fmt::Display};

pub use builder::*;
use serde::{Serialize, Serializer};

use crate::{
    core::{GetTransactionResponse, TxHash},
    middlewares::Middleware,
    providers::{JsonRpcClient, Provider},
    Error,
};

/// A high-level struct to manage a transaction.
///
/// This is the high-level representation of a transaction. It can be used to confirm a transaction.
#[derive(Debug)]
pub struct Transaction<'a, T: JsonRpcClient> {
    /// Hash of the transaction.
    pub id: TxHash,
    /// Client to confirm transaction.
    client: &'a Provider<T>,
    /// Current state of the transaction.
    status: Cell<TxStatus>,
}

#[derive(Debug, Clone, Copy)]
/// Represent the status of a transaction.
pub enum TxStatus {
    /// Transaction is initialized.
    Initialized,
    /// Transaction is not confirmed yet.
    Pending,
    /// Transaction is confirmed.
    Confirmed,
    /// Transaction is rejected.
    Rejected,
}

impl<'a, T: JsonRpcClient> Transaction<'a, T> {
    /// Creates a new Transaction.
    ///
    /// To create a new transaction you need to pass a hash, which identifies a transaction uniquely, and a
    /// provider, which is used to confirm the transaction.
    pub fn new(id: TxHash, client: &'a Provider<T>) -> Self {
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
                // TODO: Consider errors except tx hash not found.
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

/// Represents transaction version for zilliqa transactions.
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Version {
    /// Version of the message being sent
    msg_version: u16,
    /// Identifier of a blockchain network
    chain_id: u16,
}

impl Version {
    /// Create a new version instance.
    pub fn new(chain_id: u16) -> Self {
        Self {
            chain_id,
            msg_version: 1,
        }
    }

    /// Takes the `chain_id` and `msg_version` values and packs them into a
    /// single `u32` value.
    ///
    /// # Example
    /// ```
    /// use zilliqa_rs::transaction::Version;
    /// let version = Version::new(16);
    /// assert_eq!(version.pack(), 0x0010_0001)
    /// ```
    pub fn pack(&self) -> u32 {
        (self.chain_id as u32) << 16 | (self.msg_version as u32)
    }

    /// Checks if the version is valid.
    pub fn is_valid(&self) -> bool {
        (self.chain_id > 0) && (self.msg_version > 0)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "chain_id: {}, msg_version: {}", self.chain_id, self.msg_version)
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let packed = self.pack();
        serializer.serialize_u32(packed)
    }
}
