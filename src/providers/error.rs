use thiserror::Error;

use crate::{crypto::error::CryptoError, transaction::Version};

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("Version ({0}) set for the transaction is invalid")]
    InvalidVersionIsSetForTransaction(Version),

    #[error("No signers specified")]
    NoSignerSpecified,

    #[error(transparent)]
    JsonRpcError(#[from] jsonrpsee::core::Error),

    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),

    #[error(transparent)]
    CryptoError(#[from] CryptoError),
}

pub type ProviderResult<T> = Result<T, ProviderError>;
