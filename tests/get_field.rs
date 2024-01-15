mod common;
use common::TestContext;
use test_context::test_context;
use zilliqa_rs::{
    contract,
    core::{BNum, ZilAddress},
};

#[test_context(TestContext)]
#[tokio::test]
async fn set_get_scenario(ctx: &TestContext) -> anyhow::Result<()> {
    let addr = "0x1234567890123456789012345678901234567890".parse::<ZilAddress>()?;
    let provider = ctx.provider();
    let contract = contract::GetFields::deploy(provider.clone(), addr.clone(), 1000).await?;
    println!("{}", contract.address());
    let value = contract.field_uint32().await?;
    assert_eq!(value, 0);

    let value = contract.field_uint64().await?;
    assert_eq!(value, 1);

    let value = contract.field_uint128().await?;
    assert_eq!(value, 2);

    let value = contract.field_uint256().await?;
    assert_eq!(value, primitive_types::U256::from_dec_str("3")?);

    let value = contract.field_int32().await?;
    assert_eq!(value, -1);

    let value = contract.field_int64().await?;
    assert_eq!(value, -4);

    let value = contract.field_int128().await?;
    assert_eq!(value, -2);

    let value = contract.field_bnum().await?;
    assert_eq!(value, BNum::new("101"));

    let value = contract.field_string().await?;
    assert_eq!(value, "test");

    let value = contract.field_address().await?;
    assert_eq!(value, "0x1234567890123456789012345678901234567890".parse::<ZilAddress>()?);

    let value = contract.field_bool_false().await?;
    assert_eq!(value, false);

    let value = contract.field_bool_true().await?;
    assert_eq!(value, true);

    let value = contract.field_option_bystr20_none().await?;
    assert_eq!(value, None);

    let value = contract.field_option_int32_some().await?;
    assert_eq!(value, Some(10));

    let value = contract.field_option_bystr20_some().await?;
    assert_eq!(
        value,
        Some("0x1234567890123456789012345678901234567890".parse::<ZilAddress>()?)
    );

    let value = contract.field_option_bool_some().await?;
    assert_eq!(value, Some(true));

    let value = contract.field_pair().await?;
    assert_eq!(value, ("Hello".to_string(), 2));

    let value = contract.balances().await?;
    assert_eq!(value.get(&addr).unwrap(), &1000u128);

    let value = contract.field_list().await?;
    assert_eq!(value, vec![2, 1]);
    Ok(())
}
