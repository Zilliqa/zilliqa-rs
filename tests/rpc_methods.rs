use std::str::FromStr;

use claim::assert_ok;
use paste::paste;
use zilliqa_rs::core::TxHash;
use zilliqa_rs::core::ZilAddress;
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
    ($rpc_method:expr, $param1:expr, $param2:expr, $param3:expr) => {
        paste! {
            #[tokio::test]
                async fn [<$rpc_method:snake _should_work_fine>]() {
                let provider = Provider::<Http>::try_from(END_POINT).unwrap();
                assert_ok!(provider.[<$rpc_method:snake>]($param1, $param2, $param3).await);
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
rpc_method_test!(
    GetTransactionStatus,
    &TxHash::from_str("d95f28e4585220fb2f368cfa4ddcc0890b7ac0a90a3e8735ab29aeaf554f9f66").unwrap()
);

rpc_method_test!(
    GetTransaction,
    &TxHash::from_str("d95f28e4585220fb2f368cfa4ddcc0890b7ac0a90a3e8735ab29aeaf554f9f66").unwrap()
);

rpc_method_test!(
    GetContractAddressFromTransactionId,
    &TxHash::from_str("5592035396c7a5b160dbe99e0634a315e0f5fbd07366a4c1785836803ed96b9c").unwrap()
);

rpc_method_test!(
    GetSmartContractCode,
    &"988d047d9224412f76e61568f80016f8880ea898".parse::<ZilAddress>().unwrap()
);

rpc_method_test!(
    GetSmartContracts,
    &"zil1fxxz8dfk2t693eum4q3d5pmy3nwp5asysx27lm".parse::<ZilAddress>().unwrap()
);

rpc_method_test!(
    GetSmartContractInit,
    &"988d047d9224412f76e61568f80016f8880ea898".parse::<ZilAddress>().unwrap()
);

// FIXME: Needs a proper return type
rpc_method_test!(
    GetSmartContractState,
    &"988d047d9224412f76e61568f80016f8880ea898".parse::<ZilAddress>().unwrap()
);

rpc_method_test!(
    GetSmartContractSubState,
    &"988d047d9224412f76e61568f80016f8880ea898".parse::<ZilAddress>().unwrap(),
    "config_bystr20",
    &["cookie_jar_v1"]
);

rpc_method_test_ignored!(GetStateProof, "Add a test with proper parameters to the API");
