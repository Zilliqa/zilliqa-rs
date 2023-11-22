use claim::assert_ok;
use paste::paste;
use zilliqa_rs::middlewares::Middleware;
use zilliqa_rs::providers::{Http, Provider};

const END_POINT: &str = "https://api.zilliqa.com";

macro_rules! rpc_method_test {
    ($rpc_method:expr) => {
        paste! {
            #[tokio::test]
                async fn [<$rpc_method:snake _should_work_fine>]() {
                let provider = Provider::<Http>::try_from(END_POINT).unwrap();
                assert_ok!(provider.[<$rpc_method:snake>]().await);
            }
        }
    };
    ($rpc_method:expr, $param:expr) => {
        paste! {
            #[tokio::test]
                async fn [<$rpc_method:snake _should_work_fine>]() {
                let provider = Provider::<Http>::try_from(END_POINT).unwrap();
                assert_ok!(provider.[<$rpc_method:snake>]($param).await);
            }
        }
    };
    ($rpc_method:expr, $param1:expr, $param2:expr) => {
        paste! {
            #[tokio::test]
                async fn [<$rpc_method:snake _should_work_fine>]() {
                let provider = Provider::<Http>::try_from(END_POINT).unwrap();
                assert_ok!(provider.[<$rpc_method:snake>]($param1, $param2).await);
            }
        }
    };
}

macro_rules! rpc_method_test_ignored {
    ($rpc_method:expr, $ignore_msg:expr) => {
        paste! {
            #[ignore = $ignore_msg]
            #[tokio::test]
            async fn [<$rpc_method:snake _should_work_fine>]() {
            }
        }
    };
}

rpc_method_test!(GetBalance, "zil12mawdph2r00wys4q68jfsay2jy374lv6c3j5ek");
rpc_method_test!(GetBlockchainInfo);
rpc_method_test!(GetShardingStructure);
rpc_method_test!(GetDsBlock, "1");
rpc_method_test!(GetLatestDsBlock);
rpc_method_test!(GetNumDsBlocks);
rpc_method_test!(GetDsBlockRate);
rpc_method_test!(DsBlockListing, 1);
rpc_method_test!(GetTxBlock, "1");
rpc_method_test!(GetLatestTxBlock);
rpc_method_test!(GetNumTxBlocks);
rpc_method_test!(GetTxBlockRate);
rpc_method_test!(TxBlockListing, 1);
rpc_method_test!(GetNumTransactions);
rpc_method_test!(GetTransactionRate);
rpc_method_test!(GetCurrentMiniEpoch);
rpc_method_test!(GetCurrentDsEpoch);
rpc_method_test!(GetPrevDifficulty);
rpc_method_test!(GetPrevDsDifficulty);
rpc_method_test!(GetTotalCoinSupply);
rpc_method_test!(GetMinerInfo, "5500");
rpc_method_test!(GetRecentTransactions);
rpc_method_test!(GetTransactionsForTxBlock, "3357911");
rpc_method_test!(GetTransactionsForTxBlockEx, "3357911", "0");
rpc_method_test!(GetTxnBodiesForTxBlock, "3357911");
rpc_method_test!(GetTxnBodiesForTxBlockEx, "3357911", "0");
rpc_method_test!(GetNumTxnsTxEpoch, "1");
rpc_method_test!(GetNumTxnsDsEpoch, "1");
rpc_method_test!(GetMinimumGasPrice);
rpc_method_test_ignored!(GetContractAddressFromTransactionID, "not implemented");
