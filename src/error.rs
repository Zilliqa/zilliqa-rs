//! Different error variations that are possible in zilliqa-rs

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

    #[error("Default account is not set for the wallet")]
    DefaultAccountIsNotSet,

    #[error("Unknown units: {0}")]
    UnrecognizedUnits(String),

    #[error("Negative values are not allowed")]
    NegativeValueNotAllowed,

    #[error("Parse overflow")]
    ParseOverflow,

    #[error("Field {0} doesn't exist in the contract.")]
    NoSuchFieldInContractState(String),

    #[error("Field {0} doesn't exist in the contract init.")]
    NoSuchFieldInContractInit(String),

    #[error("Failed to parse scilla value {0} as {1} type")]
    FailedToParseScillaValue(String, String),

    #[error("Failed to parse {0}.")]
    FailedToParseContractField(String),

    #[error(transparent)]
    JsonRpcError(#[from] jsonrpsee::core::ClientError),

    #[error(transparent)]
    FromHexError(#[from] hex::FromHexError),

    #[error(transparent)]
    K256k1Error(#[from] k256::elliptic_curve::Error),

    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),

    #[error(transparent)]
    Bech32Error(#[from] bech32::Error),

    #[error(transparent)]
    JsonError(#[from] serde_json::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
}
