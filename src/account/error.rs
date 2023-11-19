use thiserror::Error;

use crate::crypto::error::CryptoError;

#[derive(Debug, Error)]
pub enum AccountError {
    #[error("Account with address {0} does not exist")]
    AccountDoesNotExist(String),

    #[error("Neither a pubkey nor a default account is provided")]
    NeitherPubKeyNorDefaultAccountProvided,

    #[error(transparent)]
    CryptoError(#[from] CryptoError),

    #[error(transparent)]
    JsonRpcError(#[from] jsonrpsee::core::Error),
}
