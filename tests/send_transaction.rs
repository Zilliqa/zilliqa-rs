// use zilliqa_rs::{
//     account::{Account, TransactionBuilder},
//     crypto::to_checksum_address,
//     zilliqa::Zilliqa,
// };

// #[tokio::test]
// async fn send_transaction() {
//     let zilliqa = Zilliqa::new("http://127.0.0.1:5555", 1).unwrap();
//     zilliqa
//         .wallet
//         .borrow_mut()
//         .add_by_private_key("d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba")
//         .unwrap();

//     let account = Account::create_random().unwrap();

//     let tx = TransactionBuilder::default()
//         .to_address(&to_checksum_address(&account.address).unwrap())
//         .amount(200u128 * 10u128.pow(12))
//         .gas_price(2000000000u128)
//         .gas_limit(50u64)
//         .build();

//     let res = zilliqa.blockchain.send_transaction(tx).await.unwrap();
//     println!("{:?}", res);

//     let res = zilliqa
//         .blockchain
//         .get_balance(&account.address)
//         .await
//         .unwrap();
//     println!("{:?}", res);
// }
