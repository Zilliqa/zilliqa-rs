use thiserror::Error;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("{0} is not a valid base-16 address")]
    InvalidAddress(String),

    #[error("Private key is not correct")]
    IncorrectPrivateKey,

    #[error("Private key is not verified")]
    UnverifiedPrivateKey,

    #[error(transparent)]
    FromHexError(#[from] hex::FromHexError),

    #[error(transparent)]
    Secp256k1Error(#[from] secp256k1::Error),

    #[error(transparent)]
    Bech32Error(#[from] bech32::Error),
}
