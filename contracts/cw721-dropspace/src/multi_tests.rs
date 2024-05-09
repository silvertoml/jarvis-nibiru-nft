use cosmwasm_std::{Addr, Empty};
use cw_multi_test::{App, Contract, ContractWrapper, Executor};

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