use claim::assert_ok;
use paste::paste;
use zilliqa_rs::providers::{Http, Provider};

macro_rules! rpc_method_test {
    ($rpc_method:expr) => {
        paste! {
            #[tokio::test]
                async fn [<$rpc_method:snake _should_work_fine>]() {
                let provider = Provider::<Http>::try_from("https://api.devnet.zilliqa.com").unwrap();
                assert_ok!(provider.[<$rpc_method:snake>]().await);
            }
        }
    };
    ($rpc_method:expr, $param:expr) => {
        paste! {
            #[tokio::test]
                async fn [<$rpc_method:snake _should_work_fine>]() {
                let provider = Provider::<Http>::try_from("https://api.devnet.zilliqa.com").unwrap();
                assert_ok!(provider.[<$rpc_method:snake>]($param).await);
            }
        }
    };
}

rpc_method_test!(GetBlockchainInfo);
rpc_method_test!(GetShardingStructure);
rpc_method_test!(GetDsBlock, "1");
rpc_method_test!(GetLatestDsBlock);
rpc_method_test!(GetNumDSBlocks);
rpc_method_test!(GetDSBlockRate);
rpc_method_test!(DSBlockListing, 1);
rpc_method_test!(GetTxBlock, "1");
rpc_method_test!(GetLatestTxBlock);
rpc_method_test!(GetNumTxBlocks);
rpc_method_test!(GetTxBlockRate);
rpc_method_test!(TxBlockListing, 1);
rpc_method_test!(GetNumTransactions);
rpc_method_test!(GetTransactionRate);
rpc_method_test!(GetCurrentMiniEpoch);
rpc_method_test!(GetCurrentDSEpoch);
rpc_method_test!(GetPrevDifficulty);
rpc_method_test!(GetPrevDSDifficulty);
rpc_method_test!(GetTotalCoinSupply);
rpc_method_test!(GetMinerInfo, "100");
rpc_method_test!(GetRecentTransactions);
// rpc_method_test!(GetTransactionsForTxBlock, "2");
// rpc_method_test!(GetTransactionsForTxBlockEx, tx_block, usize, Vec<Vec<String>>);
// rpc_method_test!(GetTxnBodiesForTxBlock, tx_block, usize, TransactionObj);
// rpc_method_test!(GetTxnBodiesForTxBlockEx, tx_block, usize, TransactionObj);
rpc_method_test!(GetNumTxnsTxEpoch, "1");
rpc_method_test!(GetNumTxnsDSEpoch, "1");
rpc_method_test!(GetMinimumGasPrice);
// rpc_method_test!(GetContractAddressFromTransactionID, tx_hash, String, String);
