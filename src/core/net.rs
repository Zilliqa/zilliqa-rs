pub enum RPCMethod {
    // Network-related methods
    GetNetworkId = "GetNetworkId",

    // Blockchain-related methods
    GetBlockchainInfo = "GetBlockchainInfo",
    GetShardingStructure = "GetShardingStructure",
    GetDSBlock = "GetDsBlock",
    GetLatestDSBlock = "GetLatestDsBlock",
    GetNumDSBlocks = "GetNumDSBlocks",
    GetDSBlockRate = "GetDSBlockRate",
    DSBlockListing = "DSBlockListing",
    GetTxBlock = "GetTxBlock",
    GetLatestTxBlock = "GetLatestTxBlock",
    GetNumTxBlocks = "GetNumTxBlocks",
    GetTxBlockRate = "GetTxBlockRate",
    TxBlockListing = "TxBlockListing",
    GetNumTransactions = "GetNumTransactions",
    GetTransactionRate = "GetTransactionRate",
    GetCurrentMiniEpoch = "GetCurrentMiniEpoch",
    GetCurrentDSEpoch = "GetCurrentDSEpoch",
    GetPrevDifficulty = "GetPrevDifficulty",
    GetPrevDSDifficulty = "GetPrevDSDifficulty",
    GetTotalCoinSupply = "GetTotalCoinSupply",
    GetMinerInfo = "GetMinerInfo",

    // Transaction-related methods
    CreateTransaction = "CreateTransaction",
    GetTransaction = "GetTransaction",
    GetTransactionStatus = "GetTransactionStatus",
    GetRecentTransactions = "GetRecentTransactions",
    GetTransactionsForTxBlock = "GetTransactionsForTxBlock",
    GetTransactionsForTxBlockEx = "GetTransactionsForTxBlockEx",
    GetTxnBodiesForTxBlock = "GetTxnBodiesForTxBlock",
    GetTxnBodiesForTxBlockEx = "GetTxnBodiesForTxBlockEx",
    GetNumTxnsTxEpoch = "GetNumTxnsTxEpoch",
    GetNumTxnsDSEpoch = "GetNumTxnsDSEpoch",
    GetMinimumGasPrice = "GetMinimumGasPrice",

    // Contract-related methods
    GetContractAddressFromTransactionID = "GetContractAddressFromTransactionID",
    GetSmartContracts = "GetSmartContracts",
    GetSmartContractCode = "GetSmartContractCode",
    GetSmartContractInit = "GetSmartContractInit",
    GetSmartContractState = "GetSmartContractState",
    GetSmartContractSubState = "GetSmartContractSubState",
    GetStateProof = "GetStateProof",

    // Account-related methods
    GetBalance = "GetBalance",
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
