use thiserror::Error;

use crate::crypto::error::CryptoError;

#[derive(Debug, Error)]
pub enum SignerError {
    #[error("Account with address {0} does not exist")]
    AccountDoesNotExist(String),

    #[error("Neither a pubkey nor a default account is provided")]
    NeitherPubKeyNorDefaultAccountProvided,

    #[error(transparent)]
    CryptoError(#[from] CryptoError),

    #[error(transparent)]
    JsonRpcError(#[from] jsonrpsee::core::Error),

    #[error(transparent)]
    FromHexError(#[from] hex::FromHexError),

    #[error(transparent)]
    K256k1Error(#[from] k256::elliptic_curve::Error),
}

pub type SignerResult<T> = Result<T, SignerError>;
