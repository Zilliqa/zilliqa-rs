use thiserror::Error;

use crate::{providers::ProviderError, signers::error::SignerError};

#[derive(Debug, Error)]
pub enum MiddlewareError {
    #[error("No signer exists in middlewares to catch and send the transaction")]
    NoSignerCaughtTheSendRequest,

    #[error(transparent)]
    ProviderError(#[from] ProviderError),

    #[error(transparent)]
    SignerError(#[from] SignerError),
}

pub type MiddlewareResult<T> = Result<T, MiddlewareError>;
