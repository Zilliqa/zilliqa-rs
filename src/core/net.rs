pub enum RPCMethod {
    // Network-related methods
    GetNetworkId,

    // Blockchain-related methods
    GetBlockchainInfo,
    GetShardingStructure,
    GetDSBlock,
    GetLatestDSBlock,
    GetNumDSBlocks,
    GetDSBlockRate,
    DSBlockListing,
    GetTxBlock,
    GetLatestTxBlock,
    GetNumTxBlocks,
    GetTxBlockRate,
    TxBlockListing,
    GetNumTransactions,
    GetTransactionRate,
    GetCurrentMiniEpoch,
    GetCurrentDSEpoch,
    GetPrevDifficulty,
    GetPrevDSDifficulty,
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
    GetNumTxnsDSEpoch,
    GetMinimumGasPrice,

    // Contract-related methods
    GetContractAddressFromTransactionID,
    GetSmartContracts,
    GetSmartContractCode,
    GetSmartContractInit,
    GetSmartContractState,
    GetSmartContractSubState,
    GetStateProof,

    // Account-related methods
    GetBalance,
}

impl ToString for RPCMethod {
    fn to_string(&self) -> String {
        let str = match self {
            RPCMethod::GetNetworkId => "GetNetworkId",
            RPCMethod::GetBlockchainInfo => "GetBlockchainInfo",
            RPCMethod::GetShardingStructure => "GetShardingStructure",
            RPCMethod::GetDSBlock => "GetDSBlock",
            RPCMethod::GetLatestDSBlock => "GetLatestDSBlock",
            RPCMethod::GetNumDSBlocks => "GetNumDSBlocks",
            RPCMethod::GetDSBlockRate => "GetDSBlockRate",
            RPCMethod::DSBlockListing => "DSBlockListing",
            RPCMethod::GetTxBlock => "GetTxBlock",
            RPCMethod::GetLatestTxBlock => "GetLatestTxBlock",
            RPCMethod::GetNumTxBlocks => "GetNumTxBlocks",
            RPCMethod::GetTxBlockRate => "GetTxBlockRate",
            RPCMethod::TxBlockListing => "TxBlockListing",
            RPCMethod::GetNumTransactions => "GetNumTransactions",
            RPCMethod::GetTransactionRate => "GetTransactionRate",
            RPCMethod::GetCurrentMiniEpoch => "GetCurrentMiniEpoch",
            RPCMethod::GetCurrentDSEpoch => "GetCurrentDSEpoch",
            RPCMethod::GetPrevDifficulty => "GetPrevDifficulty",
            RPCMethod::GetPrevDSDifficulty => "GetPrevDSDifficulty",
            RPCMethod::GetTotalCoinSupply => "GetTotalCoinSupply",
            RPCMethod::GetMinerInfo => "GetMinerInfo",
            RPCMethod::CreateTransaction => "CreateTransaction",
            RPCMethod::GetTransaction => "GetTransaction",
            RPCMethod::GetTransactionStatus => "GetTransactionStatus",
            RPCMethod::GetRecentTransactions => "GetRecentTransactions",
            RPCMethod::GetTransactionsForTxBlock => "GetTransactionsForTxBlock",
            RPCMethod::GetTransactionsForTxBlockEx => "GetTransactionsForTxBlockEx",
            RPCMethod::GetTxnBodiesForTxBlock => "GetTxnBodiesForTxBlock",
            RPCMethod::GetTxnBodiesForTxBlockEx => "GetTxnBodiesForTxBlockEx",
            RPCMethod::GetNumTxnsTxEpoch => "GetNumTxnsTxEpoch",
            RPCMethod::GetNumTxnsDSEpoch => "GetNumTxnsDSEpoch",
            RPCMethod::GetMinimumGasPrice => "GetMinimumGasPrice",
            RPCMethod::GetContractAddressFromTransactionID => "GetContractAddressFromTransactionID",
            RPCMethod::GetSmartContracts => "GetSmartContracts",
            RPCMethod::GetSmartContractCode => "GetSmartContractCode",
            RPCMethod::GetSmartContractInit => "GetSmartContractInit",
            RPCMethod::GetSmartContractState => "GetSmartContractState",
            RPCMethod::GetSmartContractSubState => "GetSmartContractSubState",
            RPCMethod::GetStateProof => "GetStateProof",
            RPCMethod::GetBalance => "GetBalance",
        };
        str.to_string()
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
