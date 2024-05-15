use std::f64::consts::E;

use cosmwasm_std::{coin, coins, Addr, Coin, Empty, OwnedDeps};
use cw_multi_test::{App, Contract, ContractWrapper, Executor, Router};
use cw721::NumTokensResponse;
use crate::{
    entry::{execute, instantiate, query},
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg, StatesResponse}
};
fn cw721_base_latest_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::entry::execute,
        crate::entry::instantiate,
        crate::entry::query,
    )
    .with_migrate(crate::entry::migrate);
    Box::new(contract)
}

/// Test backward compatibility using instantiate msg from a 0.16 version on latest contract.
/// This ensures existing 3rd party contracts doesnt need to updated as well.
#[test]
fn test_instantiate_016_msg() {
    use cw721_base_016 as v16;
    let mut app = App::default();
    let admin = || Addr::unchecked("admin");

    let code_id_latest = app.store_code(cw721_base_latest_contract());

    let cw721 = app
        .instantiate_contract(
            code_id_latest,
            admin(),
            &v16::InstantiateMsg {
                name: "collection".to_string(),
                symbol: "symbol".to_string(),
                minter: admin().into_string(),
            },
            &[],
            "cw721-base",
            Some(admin().into_string()),
        )
        .unwrap();

    // assert withdraw address is None
    let withdraw_addr: Option<String> = app
        .wrap()
        .query_wasm_smart(cw721, &crate::QueryMsg::<Empty>::GetWithdrawAddress {})
        .unwrap();
    assert!(withdraw_addr.is_none());
}

#[test]
fn test_buy_check_balance() {
    let mut app = App::new(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &Addr::unchecked("random"), coins(700, "unibi"))
            .unwrap()
    });

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));
    let owner = Addr::unchecked("owner");

    let instantiate_msg = InstantiateMsg {
        name: "Magic Contract".to_string(),
        symbol: "Magic".to_string(),
        minter: Some("owner".to_string()),
        base_uri: Some("https://".to_string()),
        token_id_base: Some("Magic".to_string()),
        withdraw_address: Some("anna".to_string()),
        mint_per_tx: Some(2u64),
        mint_fee: Some(100),
        dev_fee: Some(200),
        supply_limit: Some(10000),
        reserved_amount: Some(0),
        dev_wallet: Some("john".to_string()),
        sale_time: Some(0),
    };

    let addr = app.instantiate_contract(code_id, owner.clone(), &instantiate_msg, &[], "Contract", None).unwrap();
    
    /* attempt with 700 unibi and 2 qty. mint_fee: 100, dev_fee: 200, mint_per_tx: 1, withdraw_address: 'anna', dev_wallet: 'john' */
    let random = Addr::unchecked("random");
    let res = app.execute_contract(random.clone(), addr.clone(), &ExecuteMsg::<Empty, Empty>::Buy { qty: 2, extension: Empty::default() }, &coins(700, "unibi")).unwrap();

    println!("{:?}", res);
    let resp: NumTokensResponse = app
        .wrap()
        .query_wasm_smart(addr.clone(), &QueryMsg::<NumTokensResponse>::NumTokens {  })
        .unwrap();
    
    assert_eq!(
        resp,
        NumTokensResponse {
            count: 2
        }
    );

    /* check balance of 'random' is 100 because 100 is refunded */
    assert_eq!(
        app.wrap()
            .query_balance("random", "unibi")
            .unwrap()
            .amount
            .u128(),
        100
    );

    /* check balance of 'anna' is 200 because qty is 2 and mint_fee is 100 */
    assert_eq!(
        app.wrap()
            .query_balance("anna", "unibi")
            .unwrap()
            .amount
            .u128(),
        200
    );

    /* check balance of 'john' is 400 because qty is 2 and mint_fee is 200 */
    assert_eq!(
        app.wrap()
            .query_balance("john", "unibi")
            .unwrap()
            .amount
            .u128(),
        400
    );
}