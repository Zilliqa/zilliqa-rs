use thiserror::Error;

use crate::crypto::error::CryptoError;

#[derive(Debug, Error)]
pub enum AccountError {
    #[error(transparent)]
    CryptoError(#[from] CryptoError),
}
