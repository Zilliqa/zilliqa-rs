pub mod builder;
pub mod version;

use std::cell::Cell;

pub use builder::*;
pub use version::*;

use crate::{
    middlewares::Middleware,
    providers::{GetTransactionResponse, JsonRpcClient, Provider},
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

    pub async fn confirm(&self) -> Result<GetTransactionResponse, Error> {
        self.try_confirm(tokio::time::Duration::from_secs(10), 33).await
    }

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
