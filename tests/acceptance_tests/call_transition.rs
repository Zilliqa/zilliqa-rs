use crate::common::TestContext;
use test_context::test_context;
use zilliqa_rs::{contract, crypto::ZilAddress};

#[test_context(TestContext)]
#[tokio::test]
async fn set_get_scenario(ctx: &TestContext) -> anyhow::Result<()> {
    let addr = "988d047d9224412f76e61568f80016f8880ea898".parse::<ZilAddress>()?;
    let provider = ctx.provider();
    let contract = contract::CallTransition::deploy(provider.clone()).await?;

    contract.call_uint_32(1).call().await?;
    contract.call_uint_64(2).call().await?;
    contract.call_uint_128(3).call().await?;
    contract
        .call_uint_256(primitive_types::U256::from_dec_str("234").unwrap())
        .call()
        .await?;
    contract.call_int_32(5).call().await?;
    contract.call_int_64(5).call().await?;
    contract.call_int_128(6).call().await?;
    contract.call_string("hello".to_string()).call().await?;
    contract.call_address(addr.clone()).call().await?;
    contract.call_option_bool(Some(true)).call().await?;
    contract.call_bool(true).call().await?;
    contract.call_pair(("hey".to_string(), 123u32)).call().await?;
    contract.call_list(vec![]).call().await?;

    contract.call_list(vec![addr.clone()]).call().await?;

    contract.call_list_2(vec![]).call().await?;
    contract
        .call_list_2(vec![(addr.clone(), vec![(addr.clone(), 32)])])
        .call()
        .await?;

    contract
        .call_list_3(vec![("khar".to_string(), "gav".to_string())])
        .call()
        .await?;

    contract.call_list_4(vec![]).call().await?;
    contract
        .call_list_4(vec![(addr.clone(), vec![(addr.clone(), vec![(23, 32)])])])
        .call()
        .await?;

    Ok(())
}
