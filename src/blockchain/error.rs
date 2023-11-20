use thiserror::Error;

use crate::{account::error::AccountError, crypto::error::CryptoError};

#[derive(Debug, Error)]
pub enum BlockchainError {
    #[error(transparent)]
    CryptoError(#[from] CryptoError),

    #[error(transparent)]
    JsonRpcError(#[from] jsonrpsee::core::Error),

    #[error(transparent)]
    AccountError(#[from] AccountError),
}

pub type ChainResult<T> = Result<T, BlockchainError>;
