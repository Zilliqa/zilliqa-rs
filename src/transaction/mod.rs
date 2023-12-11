pub mod builder;
pub mod version;

use std::cell::{Cell, RefCell};

pub use builder::*;
pub use version::*;

use crate::{
    middlewares::Middleware,
    providers::{JsonRpcClient, Provider, TransactionReceipt},
    Error,
};

#[derive(Debug)]
pub struct Transaction<'a, T: JsonRpcClient> {
    pub id: String,
    client: &'a Provider<T>,
    status: Cell<TxStatus>,
    receipt: RefCell<TransactionReceipt>,
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
            receipt: RefCell::new(TransactionReceipt::default()),
        }
    }

    pub async fn confirm(&self, interval: tokio::time::Duration, max_attempt: u32) -> Result<(), Error> {
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

            *self.receipt.borrow_mut() = res.receipt;
            return Ok(());
        }

        Err(Error::UnableToConfirmTransaction(max_attempt))
    }

    pub async fn receipt(&self) -> Result<&RefCell<TransactionReceipt>, Error> {
        match self.status.get() {
            TxStatus::Initialized | TxStatus::Pending => {
                self.confirm(tokio::time::Duration::from_secs(10), 33).await?;
            }
            _ => {}
        };

        Ok(&self.receipt)
    }
}
