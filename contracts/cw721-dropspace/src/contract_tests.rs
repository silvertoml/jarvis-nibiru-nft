#![cfg(test)]
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

use cosmwasm_std::{
    coin, DepsMut, Empty
};

use cw721::{
    ContractInfoResponse, Cw721Query, 
};
use cw_ownable::OwnershipError;

use crate::{
    ContractError, Cw721Contract, ExecuteMsg, Extension, InstantiateMsg
};

const MINTER: &str = "merlin";
const CONTRACT_NAME: &str = "Magic Power";
const SYMBOL: &str = "MGK";
const BASE_URI: &str = "";
const TOKEN_ID_BASE: &str = "";

fn setup_contract(deps: DepsMut<'_>) -> Cw721Contract<'static, Extension, Empty, Empty, Empty> {
    let contract = Cw721Contract::default();
    let msg = InstantiateMsg {
        name: CONTRACT_NAME.to_string(),
        symbol: SYMBOL.to_string(),
        base_uri: Some(BASE_URI.to_string()),
        token_id_base: Some(TOKEN_ID_BASE.to_string()),
        minter: Some(String::from(MINTER)),
        withdraw_address: Some(String::from(MINTER)),
        mint_per_tx: Some(100u64),
        mint_fee: Some(0u64),
        dev_fee: Some(0u64),
        supply_limit: Some(10000u64),
        reserved_amount: Some(0u64),
        dev_wallet: None,
        sale_time: None
    };
    let info = mock_info("creator", &[]);
    let res = contract.instantiate(deps, mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());
    contract
}

#[test]
fn proper_instantiation() {
    let mut deps = mock_dependencies();
    let contract = Cw721Contract::<Extension, Empty, Empty, Empty>::default();

    let msg = InstantiateMsg {
        name: CONTRACT_NAME.to_string(),
        symbol: SYMBOL.to_string(),
        base_uri: Some(BASE_URI.to_string()),
        token_id_base: Some(TOKEN_ID_BASE.to_string()),
        minter: Some(String::from(MINTER)),
        withdraw_address: Some(String::from(MINTER)),
        mint_per_tx: Some(100u64),
        mint_fee: Some(0u64),
        dev_fee: Some(0u64),
        supply_limit: Some(10000u64),
        reserved_amount: Some(0u64),
        dev_wallet: None,
        sale_time: None
    };
    let info = mock_info("creator", &[]);

    // we can just call .unwrap() to assert this was a success
    let res = contract
        .instantiate(deps.as_mut(), mock_env(), info, msg)
        .unwrap();
    assert_eq!(0, res.messages.len());

    // it worked, let's query the state
    let res = contract.minter(deps.as_ref()).unwrap();
    assert_eq!(Some(MINTER.to_string()), res.minter);
    let info = contract.contract_info(deps.as_ref()).unwrap();
    assert_eq!(
        info,
        ContractInfoResponse {
            name: CONTRACT_NAME.to_string(),
            symbol: SYMBOL.to_string(),
        }
    );

    let withdraw_address = contract
        .withdraw_address
        .may_load(deps.as_ref().storage)
        .unwrap();
    assert_eq!(Some(MINTER.to_string()), withdraw_address);

    let count = contract.num_tokens(deps.as_ref()).unwrap();
    assert_eq!(0, count.count);

    // list the token_ids
    let tokens = contract.all_tokens(deps.as_ref(), None, None).unwrap();
    assert_eq!(0, tokens.tokens.len());
}

#[test]
fn set_get_dev_wallet() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());

    let set_dev_wallet_msg = ExecuteMsg::SetDevWallet { address: "recipient".to_string() };

    // random cannot
    let random = mock_info("random", &[]);

    let err = contract
        .execute(deps.as_mut(), mock_env(), random, set_dev_wallet_msg.clone())
        .unwrap_err();
    assert_eq!(err, ContractError::Ownership(OwnershipError::NotOwner));
    
    // owner can
    let owner = mock_info("merlin", &[]);

    contract
        .execute(deps.as_mut(), mock_env(), owner, set_dev_wallet_msg)
        .unwrap();

    let res = contract.dev_wallet.may_load(deps.as_ref().storage).unwrap_or_default();
    assert_eq!(Some("recipient".to_string()), res);
}

#[test]
fn set_get_base_uri() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());

    let set_base_uri_msg = ExecuteMsg::SetBaseUri { base_uri: "https://randomUri/".into() };

    // random cannot
    let random = mock_info("random", &[]);

    let err = contract
        .execute(deps.as_mut(), mock_env(), random, set_base_uri_msg.clone())
        .unwrap_err();
    assert_eq!(err, ContractError::Ownership(OwnershipError::NotOwner));
    
    // owner can
    let owner = mock_info("merlin", &[]);

    contract
        .execute(deps.as_mut(), mock_env(), owner, set_base_uri_msg)
        .unwrap();

    let res = contract.base_uri.may_load(deps.as_ref().storage).unwrap_or_default();
    assert_eq!(Some("https://randomUri/".into()), res);
}

#[test]
fn set_get_mint_per_tx() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());

    let set_mint_per_tx_msg = ExecuteMsg::SetMintPerTx { tx: 100u64 } ;

    // random cannot
    let random = mock_info("random", &[]);

    let err = contract
        .execute(deps.as_mut(), mock_env(), random, set_mint_per_tx_msg.clone())
        .unwrap_err();
    assert_eq!(err, ContractError::Ownership(OwnershipError::NotOwner));
    
    // owner can
    let owner = mock_info("merlin", &[]);

    contract
        .execute(deps.as_mut(), mock_env(), owner, set_mint_per_tx_msg)
        .unwrap();

    let res = contract.mint_per_tx.may_load(deps.as_ref().storage).unwrap_or_default();
    assert_eq!(Some(100u64), res);
}

#[test]
fn set_get_mint_fee() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());

    let set_mint_fee_msg = ExecuteMsg::SetMintFee { fee: 100u64 } ;

    // random cannot
    let random = mock_info("random", &[]);

    let err = contract
        .execute(deps.as_mut(), mock_env(), random, set_mint_fee_msg.clone())
        .unwrap_err();
    assert_eq!(err, ContractError::Ownership(OwnershipError::NotOwner));
    
    // owner can
    let owner = mock_info("merlin", &[]);

    contract
        .execute(deps.as_mut(), mock_env(), owner, set_mint_fee_msg)
        .unwrap();

    let res = contract.mint_fee.may_load(deps.as_ref().storage).unwrap_or_default();
    assert_eq!(Some(100u64), res);
}

#[test]
fn set_get_dev_fee() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());

    let set_dev_fee_msg = ExecuteMsg::SetDevFee { fee: 100u64 } ;

    // random cannot
    let random = mock_info("random", &[]);

    let err = contract
        .execute(deps.as_mut(), mock_env(), random, set_dev_fee_msg.clone())
        .unwrap_err();
    assert_eq!(err, ContractError::Ownership(OwnershipError::NotOwner));
    
    // owner can
    let owner = mock_info("merlin", &[]);

    contract
        .execute(deps.as_mut(), mock_env(), owner, set_dev_fee_msg)
        .unwrap();

    let res = contract.dev_fee.may_load(deps.as_ref().storage).unwrap_or_default();
    assert_eq!(Some(100u64), res);
}

#[test]
fn set_get_supply_limit() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());

    let set_supply_limit_msg = ExecuteMsg::SetSupplyLimit { supply_limit: 10000u64 } ;

    // random cannot
    let random = mock_info("random", &[]);

    let err = contract
        .execute(deps.as_mut(), mock_env(), random, set_supply_limit_msg.clone())
        .unwrap_err();
    assert_eq!(err, ContractError::Ownership(OwnershipError::NotOwner));
    
    // owner can
    let owner = mock_info("merlin", &[]);

    contract
        .execute(deps.as_mut(), mock_env(), owner, set_supply_limit_msg)
        .unwrap();

    let res = contract.supply_limit.may_load(deps.as_ref().storage).unwrap_or_default();
    assert_eq!(Some(10000u64), res);
}

#[test]
fn set_get_sale_time() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());

    let set_sale_time_msg = ExecuteMsg::SetSaleTime { sale_time: 100u64 } ;

    // random cannot
    let random = mock_info("random", &[]);

    let err = contract
        .execute(deps.as_mut(), mock_env(), random, set_sale_time_msg.clone())
        .unwrap_err();
    assert_eq!(err, ContractError::Ownership(OwnershipError::NotOwner));
    
    // owner can
    let owner = mock_info("merlin", &[]);

    contract
        .execute(deps.as_mut(), mock_env(), owner, set_sale_time_msg)
        .unwrap();

    let res = contract.sale_time.may_load(deps.as_ref().storage).unwrap_or_default();
    assert_eq!(Some(100u64), res);
}

#[test]
fn set_get_name() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());

    let set_name_msg = ExecuteMsg::SetName { name: "Name".into() } ;

    // random cannot
    let random = mock_info("random", &[]);

    let err = contract
        .execute(deps.as_mut(), mock_env(), random, set_name_msg.clone())
        .unwrap_err();
    assert_eq!(err, ContractError::Ownership(OwnershipError::NotOwner));
    
    // owner can
    let owner = mock_info("merlin", &[]);

    contract
        .execute(deps.as_mut(), mock_env(), owner, set_name_msg)
        .unwrap();

    let contract_info = contract.contract_info.load(deps.as_ref().storage).unwrap();
    assert_eq!("Name".to_string(), contract_info.name);
}

#[test]
fn set_get_symbol() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());

    let set_symbol_msg = ExecuteMsg::SetSymbol { symbol: "Symbol".into() } ;

    // random cannot
    let random = mock_info("random", &[]);

    let err = contract
        .execute(deps.as_mut(), mock_env(), random, set_symbol_msg.clone())
        .unwrap_err();
    assert_eq!(err, ContractError::Ownership(OwnershipError::NotOwner));
    
    // owner can
    let owner = mock_info("merlin", &[]);

    contract
        .execute(deps.as_mut(), mock_env(), owner, set_symbol_msg)
        .unwrap();

    let contract_info = contract.contract_info.load(deps.as_ref().storage).unwrap();
    assert_eq!("Symbol".to_string(), contract_info.symbol);
}

#[test]
fn test_buy() {
    let mut deps = mock_dependencies();
    let contract = setup_contract(deps.as_mut());
    
    let owner = mock_info("merlin", &[]);
    let random = mock_info("random", &[]);
    let random2 = mock_info("random2", &[coin(100, "unibi")]);
    let random3 = mock_info("random3", &[coin(200, "unibi")]);
    let random4 = mock_info("random4", &[coin(300, "unibi")]);
    let buy_msg = ExecuteMsg::Buy { qty: 1, extension: None };
    
    // set sale active
    contract
        .execute(deps.as_mut(), mock_env(), owner.clone(), ExecuteMsg::ToggleSaleActive {  })
        .unwrap();

    // attempt without funds, in case 0 mint_fee + 0 dev_fee
    contract
        .execute(deps.as_mut(), mock_env(), random.clone(), buy_msg.clone())
        .unwrap();

    // set mint fee to 100 unibi
    let set_mint_fee_msg = ExecuteMsg::SetMintFee { fee: 100u64 };
    
    contract
        .execute(deps.as_mut(), mock_env(), owner.clone(), set_mint_fee_msg)
        .unwrap();

    // attempt without funds, in case 100 mint_fee + 0 dev_fee
    let err = contract
        .execute(deps.as_mut(), mock_env(), random.clone(), buy_msg.clone())
        .unwrap_err();
    assert_eq!(err, ContractError::IncorrectFunds {  });

    // attempt with 100 unibi, in case 100 mint_fee + 0 dev_fee
    contract
        .execute(deps.as_mut(), mock_env(), random2.clone(), buy_msg.clone())
        .unwrap();

    // set dev fee to 200 unibi
    let set_dev_fee_msg = ExecuteMsg::SetDevFee { fee: 200u64 };
    
    contract
        .execute(deps.as_mut(), mock_env(), owner.clone(), set_dev_fee_msg)
        .unwrap();

    // attempt with 200 unibi, in case 100 mint_fee + 200 dev_fee
    let err = contract
        .execute(deps.as_mut(), mock_env(), random3.clone(), buy_msg.clone())
        .unwrap_err();
    assert_eq!(err, ContractError::IncorrectFunds {  });

    // attempt with 300 unibi, in case 100 mint_fee + 200 dev_fee
    contract
        .execute(deps.as_mut(), mock_env(), random4.clone(), buy_msg.clone())
        .unwrap();

    // set mint fee to 0 unibi
    let set_mint_fee_msg = ExecuteMsg::SetMintFee { fee: 0u64 };
    
    contract
        .execute(deps.as_mut(), mock_env(), owner.clone(), set_mint_fee_msg)
        .unwrap();

    // attempt with 100 unibi, in case 0 mint_fee + 200 dev_fee
    let err = contract
        .execute(deps.as_mut(), mock_env(), random2.clone(), buy_msg.clone())
        .unwrap_err();
    assert_eq!(err, ContractError::IncorrectFunds {  });

    // attempt with 200 unibi, in case 0 mint_fee + 200 dev_fee
    contract
        .execute(deps.as_mut(), mock_env(), random3, buy_msg.clone())
        .unwrap();
}
