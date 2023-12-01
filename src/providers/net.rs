use core::fmt;

#[derive(Debug)]
pub enum RPCMethod {
    // Network-related methods
    GetNetworkId,

    // Blockchain-related methods
    GetBlockchainInfo,
    GetShardingStructure,
    GetDsBlock,
    GetLatestDsBlock,
    GetNumDsBlocks,
    GetDsBlockRate,
    DsBlockListing,
    GetTxBlock,
    GetLatestTxBlock,
    GetNumTxBlocks,
    GetTxBlockRate,
    TxBlockListing,
    GetNumTransactions,
    GetTransactionRate,
    GetCurrentMiniEpoch,
    GetCurrentDsEpoch,
    GetPrevDifficulty,
    GetPrevDsDifficulty,
    GetTotalCoinSupply,
    GetMinerInfo,

    // Transaction-related methods
    CreateTransaction,
    GetTransaction,
    GetTransactionStatus,
    GetRecentTransactions,
    GetTransactionsForTxBlock,
    GetTransactionsForTxBlockEx,
    GetTxnBodiesForTxBlock,
    GetTxnBodiesForTxBlockEx,
    GetNumTxnsTxEpoch,
    GetNumTxnsDsEpoch,
    GetMinimumGasPrice,

    // Contract-related methods
    GetContractAddressFromTransactionId,
    GetSmartContracts,
    GetSmartContractCode,
    GetSmartContractInit,
    GetSmartContractState,
    GetSmartContractSubState,
    GetStateProof,

    // Account-related methods
    GetBalance,
}

impl fmt::Display for RPCMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DsBlockListing => write!(f, "DSBlockListing"),
            Self::GetNumDsBlocks => write!(f, "GetNumDSBlocks"),
            Self::GetDsBlockRate => write!(f, "GetDSBlockRate"),
            Self::GetCurrentDsEpoch => write!(f, "GetCurrentDSEpoch"),
            Self::GetPrevDsDifficulty => write!(f, "GetPrevDSDifficulty"),
            Self::GetNumTxnsDsEpoch => write!(f, "GetNumTxnsDSEpoch"),
            Self::GetContractAddressFromTransactionId => {
                write!(f, "GetContractAddressFromTransactionID")
            }
            _ => fmt::Debug::fmt(self, f),
        }
    }
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
