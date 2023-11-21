use thiserror::Error;

use crate::crypto::error::CryptoError;

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error(transparent)]
    JsonRpcError(#[from] jsonrpsee::core::Error),

    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),

    #[error(transparent)]
    CryptoError(#[from] CryptoError),
}

pub type ProviderResult<T> = Result<T, ProviderError>;
