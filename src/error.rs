use std::num::ParseIntError;

use thiserror::Error as ThisError;

use crate::transaction::Version;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Failed to confirm transaction after {0} tries.")]
    UnableToConfirmTransaction(u32),

    #[error("{0} is not a valid base-16 address")]
    InvalidAddress(String),

    #[error("Private key is not correct")]
    IncorrectPrivateKey,

    #[error("Provided txn hash {0} is invalid.")]
    InvalidTransactionHash(String),

    #[error("Version ({0}) set for the transaction is invalid")]
    InvalidVersionIsSetForTransaction(Version),

    #[error("No signers specified, unable to send/sign.")]
    NoSignerSpecified,

    #[error("Account with address {0} does not exist")]
    AccountDoesNotExist(String),

    #[error("Neither a pubkey nor a default account is provided")]
    NeitherPubKeyNorDefaultAccountProvided,

    #[error("Unknown units: {0}")]
    UnrecognizedUnits(String),

    #[error("Negative values are not allowed")]
    NegativeValueNotAllowed,

    #[error("Parse overflow")]
    ParseOverflow,

    #[error(transparent)]
    JsonRpcError(#[from] jsonrpsee::core::Error),

    #[error(transparent)]
    FromHexError(#[from] hex::FromHexError),

    #[error(transparent)]
    K256k1Error(#[from] k256::elliptic_curve::Error),

    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),

    #[error(transparent)]
    Secp256k1Error(#[from] secp256k1::Error),

    #[error(transparent)]
    Bech32Error(#[from] bech32::Error),

    #[error(transparent)]
    JsonError(#[from] serde_json::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
}
