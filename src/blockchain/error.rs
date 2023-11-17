use thiserror::Error;

use crate::crypto::error::CryptoError;

#[derive(Debug, Error)]
pub enum BlockchainError {
    #[error(transparent)]
    CryptoError(#[from] CryptoError),

    #[error(transparent)]
    JsonRpcError(#[from] jsonrpsee::core::Error),
}
