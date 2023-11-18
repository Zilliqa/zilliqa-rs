use thiserror::Error;

use crate::crypto::error::CryptoError;

#[derive(Debug, Error)]
pub enum AccountError {
    #[error("Account with address {0} does not exist")]
    AccountDoesNotExist(String),

    #[error(transparent)]
    CryptoError(#[from] CryptoError),
}
