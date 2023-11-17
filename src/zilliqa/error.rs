use thiserror::Error;

#[derive(Debug, Error)]
pub enum ZilliqaError {
    #[error(transparent)]
    CoreError(#[from] crate::core::error::CoreError),
}
