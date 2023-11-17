use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error(transparent)]
    JsonRpcError(#[from] jsonrpsee::core::Error),
}
