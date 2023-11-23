use thiserror::Error;

use crate::middlewares::MiddlewareError;

#[derive(Debug, Error)]
pub enum ContractError {
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    MiddlewareError(#[from] MiddlewareError),
}

pub type ContractResult<T> = Result<T, ContractError>;
