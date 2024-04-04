//! Clients for interacting with zilliqa network.

pub mod http;
pub mod provider;

pub use http::Http;
pub use provider::Provider;

use crate::Error;
use async_trait::async_trait;
use jsonrpsee::core::traits::ToRpcParams;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

/// JSON-RPC client trait
///
/// If a client wants to be used as a JSON-RPC client, it must implement this trait.
#[async_trait]
pub trait JsonRpcClient: Debug + Send + Sync {
    async fn request<T, R>(&self, method: &str, params: T) -> Result<R, Error>
    where
        T: Debug + Send + Sync + ToRpcParams,
        R: DeserializeOwned + Send;
}

pub enum RPCErrorCode {
    // Standard JSON-RPC 2.0 errors
    // RPC_INVALID_REQUEST is internally mapped to HTTP_BAD_REQUEST (400).
    // It should not be used for application-layer errors.
    RpcInvalidRequest = -32600,
    // RPC_METHOD_NOT_FOUND is internally mapped to HTTP_NOT_FOUND (404).
    // It should not be used for application-layer errors.
    RpcMethodNotFound = -32601,
    RpcInvalidParams = -32602,
    // RPC_INTERNAL_ERROR should only be used for genuine errors in bitcoind
    // (for example datadir corruption).
    RpcInternalError = -32603,
    RpcParseError = -32700,

    // General application defined errors
    RpcMiscError = -1,             // std::exception thrown in command handling
    RpcTypeError = -3,             // Unexpected type was passed as parameter
    RpcInvalidAddressOrKey = -5,   // Invalid address or key
    RpcInvalidParameter = -8,      // Invalid, missing or duplicate parameter
    RpcDatabaseError = -20,        // Database error
    RpcDeserializationError = -22, // Error parsing or validating structure in raw format
    RpcVerifyError = -25,          // General error during transaction or block submission
    RpcVerifyRejected = -26,       // Transaction or block was rejected by network rules
    RpcInWarmup = -28,             // Client still warming up
    RpcMethodDeprecated = -32,     // RPC method is deprecated
}
