use cw_ownable::OwnershipError;
use serde::de::DeserializeOwned;
use serde::Serialize;

use cosmwasm_std::{
    Addr, BankMsg, Binary, Coin, CustomMsg, Deps, DepsMut, Env, MessageInfo, Response, Storage, SubMsg, Empty, coin
};

use cw721::{ContractInfoResponse, Cw721Execute, Cw721ReceiveMsg, Expiration};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{Approval, Cw721Contract, TokenInfo};
use std::cmp;

impl<'a, T, C, E, Q> Cw721Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    pub fn instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response<C>, ContractError> {
        let contract_info = ContractInfoResponse {
            name: msg.name,
            symbol: msg.symbol,
        };
        self.contract_info.save(deps.storage, &contract_info)?;
        let mint_per_tx = 1u64;
        let mint_fee = 0u64;
        let dev_fee = 0u64;
        let suply_limit = 100000u64;
        let total_supply = 0u64;
        let reserved_amount = 0u64;
        let dev_wallet = info.clone().sender.to_string();
        let sale_time = 0u64;
        self.mint_per_tx.save(deps.storage, &mint_per_tx)?;
        self.mint_fee.save(deps.storage, &mint_fee)?;
        self.dev_fee.save(deps.storage, &dev_fee)?;
        self.suply_limit.save(deps.storage, &suply_limit)?;
        self.total_supply.save(deps.storage, &total_supply)?;
        self.reserved_amount.save(deps.storage, &reserved_amount)?;
        self.dev_wallet.save(deps.storage, &dev_wallet)?;
        self.sale_time.save(deps.storage, &sale_time)?;

        let owner = match msg.minter {
            Some(owner) => deps.api.addr_validate(&owner)?,
            None => info.sender,
        };
        cw_ownable::initialize_owner(deps.storage, deps.api, Some(owner.as_ref()))?;

        if let Some(address) = msg.withdraw_address {
            self.set_withdraw_address(deps, &owner, address)?;
        }
        
        Ok(Response::default())
    }

    pub fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg<T, E>,
    ) -> Result<Response<C>, ContractError> {
        match msg {
            ExecuteMsg::Mint {
                token_id,
                owner,
                token_uri,
                extension,
            } => self.mint(deps, info, token_id, 1, extension),
            ExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            } => self.approve(deps, env, info, spender, token_id, expires),
            ExecuteMsg::Revoke { spender, token_id } => {
                self.revoke(deps, env, info, spender, token_id)
            }
            ExecuteMsg::ApproveAll { operator, expires } => {
                self.approve_all(deps, env, info, operator, expires)
            }
            ExecuteMsg::RevokeAll { operator } => self.revoke_all(deps, env, info, operator),
            ExecuteMsg::TransferNft {
                recipient,
                token_id,
            } => self.transfer_nft(deps, env, info, recipient, token_id),
            ExecuteMsg::SendNft {
                contract,
                token_id,
                msg,
            } => self.send_nft(deps, env, info, contract, token_id, msg),
            ExecuteMsg::Burn { token_id } => self.burn(deps, env, info, token_id),
            ExecuteMsg::UpdateOwnership(action) => Self::update_ownership(deps, env, info, action),
            ExecuteMsg::Extension { msg: _ } => Ok(Response::default()),
            ExecuteMsg::SetWithdrawAddress { address } => self.set_withdraw_address(deps, &info.sender, address),
            ExecuteMsg::SetDevWallet { address } => self.set_dev_wallet(deps, &info.sender, address),
            ExecuteMsg::RemoveWithdrawAddress {} => {
                self.remove_withdraw_address(deps.storage, &info.sender)
            }
            ExecuteMsg::WithdrawFunds { amount } => self.withdraw_funds(deps.storage, &amount),
            ExecuteMsg::SetName { name } => self.set_name(deps.storage, &info.sender, &name),
            ExecuteMsg::SetSymbol { symbol } => self.set_symbol(deps.storage, &info.sender, &symbol),
            ExecuteMsg::SetMintPerTx { tx } => self.set_mint_per_tx(deps, &info.sender, &tx),
            ExecuteMsg::SetMintFee { fee } => self.set_mint_fee(deps, &info.sender, &fee),
            ExecuteMsg::SetDevFee { fee } => self.set_dev_fee(deps, &info.sender, &fee),
            ExecuteMsg::SetSupplyLimit { supply_limit } => self.set_supply_limit(deps, &info.sender, &supply_limit),
            ExecuteMsg::SetSaleTime { sale_time } => self.set_sale_time(deps, &info.sender, &sale_time),
            ExecuteMsg::Buy { qty , extension} => self.buy(deps, info, &qty, extension),
            ExecuteMsg::Reserve { qty, extension } => self.reserve(deps, info, &qty, extension),
            ExecuteMsg::ToggleSaleActive {  } => self.toggle_sale_active(deps, &info.sender),
        }
    }
}

// TODO pull this into some sort of trait extension??
impl<'a, T, C, E, Q> Cw721Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    pub fn mint(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        token_id: String,
        qty: u64,
        extension: T,
    ) -> Result<Response<C>, ContractError> {
        cw_ownable::assert_owner(deps.storage, &info.clone().sender)?;
        let mut total_supply = self.total_supply.may_load(deps.storage)?.unwrap_or_else(|| 0u64);
        // create the token
        for i in 0..qty.clone() {
            let token = TokenInfo {
                owner: info.clone().sender,
                approvals: vec![],
                token_uri: Some(format!("https://ipfs.io/ipfs/bafybeigrytqzipxv4sekrofqfz4etp4f6c7a3bssi5oyerccmeksm4czku/{}", total_supply.clone() + i + 1)),
                extension: extension.clone(),
            };
            self.tokens
            .update(deps.storage, &token_id, |old| match old {
                Some(_) => Err(ContractError::Claimed {}),
                None => Ok(token),
            })?;
            
            self.increment_tokens(deps.storage)?;
        }
        total_supply += qty.clone();
        self.total_supply.save(deps.storage, &total_supply)?;

        Ok(Response::new()
            .add_attribute("action", "mint")
            .add_attribute("minter", info.clone().sender)
            .add_attribute("owner", info.clone().sender)
            .add_attribute("token_id", token_id))
    }

    pub fn update_ownership(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        action: cw_ownable::Action,
    ) -> Result<Response<C>, ContractError> {
        let ownership = cw_ownable::update_ownership(deps, &env.block, &info.sender, action)?;
        Ok(Response::new().add_attributes(ownership.into_attributes()))
    }

    pub fn set_withdraw_address(
        &self,
        deps: DepsMut,
        sender: &Addr,
        address: String,
    ) -> Result<Response<C>, ContractError> {
        cw_ownable::assert_owner(deps.storage, sender)?;
        deps.api.addr_validate(&address)?;
        self.withdraw_address.save(deps.storage, &address)?;
        Ok(Response::new()
            .add_attribute("action", "set_withdraw_address")
            .add_attribute("address", address))
    }

    pub fn set_dev_wallet(
        &self,
        deps: DepsMut,
        sender: &Addr,
        address: String,
    ) -> Result<Response<C>, ContractError> {
        cw_ownable::assert_owner(deps.storage, sender)?;
        deps.api.addr_validate(&address)?;
        self.dev_wallet.save(deps.storage, &address)?;
        Ok(Response::new()
            .add_attribute("action", "set_dev_wallet")
            .add_attribute("address", address))
    }

    pub fn remove_withdraw_address(
        &self,
        storage: &mut dyn Storage,
        sender: &Addr,
    ) -> Result<Response<C>, ContractError> {
        cw_ownable::assert_owner(storage, sender)?;
        let address = self.withdraw_address.may_load(storage)?;
        match address {
            Some(address) => {
                self.withdraw_address.remove(storage);
                Ok(Response::new()
                    .add_attribute("action", "remove_withdraw_address")
                    .add_attribute("address", address))
            }
            None => Err(ContractError::NoWithdrawAddress {}),
        }
    }

    pub fn withdraw_funds(
        &self,
        storage: &mut dyn Storage,
        amount: &Coin,
    ) -> Result<Response<C>, ContractError> {
        let address = self.withdraw_address.may_load(storage)?;
        match address {
            Some(address) => {
                let msg = BankMsg::Send {
                    to_address: address,
                    amount: vec![amount.clone()],
                };
                Ok(Response::new()
                    .add_message(msg)
                    .add_attribute("action", "withdraw_funds")
                    .add_attribute("amount", amount.amount.to_string())
                    .add_attribute("denom", amount.denom.to_string()))
            }
            None => Err(ContractError::NoWithdrawAddress {}),
        }
    }

    pub fn set_name(
        &self,
        storage: &mut dyn Storage,
        sender: &Addr,
        name: &String,
    ) -> Result<Response<C>, ContractError> {
        // self.contract_info.save(deps.storage, &contract_info)?;
        // self
        cw_ownable::assert_owner(storage, sender)?;
        let old_contract_info = self.contract_info.may_load(storage)?;
        let old_name = old_contract_info.unwrap_or_else(|| ContractInfoResponse{
            name: "None".to_string(),
            symbol: "None".to_string()
        });

        let new_contract_info = ContractInfoResponse{
            symbol: old_name.symbol,
            name: name.to_string(),
        };
        
        self.contract_info.save(storage, &new_contract_info)?;
        Ok(Response::new()
            .add_attribute("action", "set_name")
            .add_attribute("name", name))
    }

    pub fn set_symbol(
        &self,
        storage: &mut dyn Storage,
        sender: &Addr,
        symbol: &String,
    ) -> Result<Response<C>, ContractError> {
        // self.contract_info.save(deps.storage, &contract_info)?;
        // self
        cw_ownable::assert_owner(storage, sender)?;
        let old_contract_info = self.contract_info.may_load(storage)?;
        let old_name = old_contract_info.unwrap_or_else(|| ContractInfoResponse{
            name: "None".to_string(),
            symbol: "None".to_string()
        });

        let new_contract_info = ContractInfoResponse{
            symbol: symbol.to_string(),
            name: old_name.name,
        };
        
        self.contract_info.save(storage, &new_contract_info)?;
        Ok(Response::new()
            .add_attribute("action", "set_symbol")
            .add_attribute("symbol", symbol))
    }

    pub fn set_mint_per_tx(
        &self,
        deps: DepsMut,
        sender: &Addr,
        tx: &u64,
    ) -> Result<Response<C>, ContractError> {
        cw_ownable::assert_owner(deps.storage, sender)?;
        
        self.mint_per_tx.save(deps.storage, &tx)?;
        Ok(Response::new()
            .add_attribute("action", "set_mint_per_tx")
            .add_attribute("mint_per_tx", tx.to_string()))
    }

    pub fn set_mint_fee(
        &self,
        deps: DepsMut,
        sender: &Addr,
        fee: &u64,
    ) -> Result<Response<C>, ContractError> {
        cw_ownable::assert_owner(deps.storage, sender)?;
        
        self.mint_fee.save(deps.storage, &fee)?;
        Ok(Response::new()
            .add_attribute("action", "set_mint_fee")
            .add_attribute("mint_fee", fee.to_string()))
    }

    pub fn set_dev_fee(
        &self,
        deps: DepsMut,
        sender: &Addr,
        fee: &u64,
    ) -> Result<Response<C>, ContractError> {
        cw_ownable::assert_owner(deps.storage, sender)?;
        
        self.dev_fee.save(deps.storage, &fee)?;
        Ok(Response::new()
            .add_attribute("action", "set_dev_fee")
            .add_attribute("dev_fee", fee.to_string()))
    }

    pub fn set_supply_limit(
        &self,
        deps: DepsMut,
        sender: &Addr,
        supply_limit: &u64,
    ) -> Result<Response<C>, ContractError> {
        cw_ownable::assert_owner(deps.storage, sender)?;
        
        self.suply_limit.save(deps.storage, &supply_limit)?;
        Ok(Response::new()
            .add_attribute("action", "set_supply_limit")
            .add_attribute("supply_limit", supply_limit.to_string()))
    }

    pub fn set_sale_time(
        &self,
        deps: DepsMut,
        sender: &Addr,
        sale_time: &u64,
    ) -> Result<Response<C>, ContractError> {
        cw_ownable::assert_owner(deps.storage, sender)?;
        
        self.sale_time.save(deps.storage, &sale_time)?;
        Ok(Response::new()
            .add_attribute("action", "set_sale_time")
            .add_attribute("sale_time", sale_time.to_string()))
    }

    pub fn buy(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        qty: &u64,
        extension: T
    ) -> Result<Response<C>, ContractError> {
        let sale_active = self.sale_active.may_load(deps.storage)?;
        if sale_active.unwrap_or_else(|| false) == false {
            return Err(ContractError::SaleUnactivate {});
        }

        let sent_funds: u128 = info.funds.iter().find(|coin| coin.denom == "unibi").map_or(0u128, |coin| coin.amount.u128());
        let mint_fee = self.mint_fee.may_load(deps.storage)?.unwrap_or_else(|| 0);
        let dev_fee = self.dev_fee.may_load(deps.storage)?.unwrap_or_else(|| 0);
        let mint_per_tx = self.mint_per_tx.may_load(deps.storage)?;
        let total_fee = mint_fee.clone() + dev_fee.clone();
        let withdraw_address = self.withdraw_address.may_load(deps.storage)?.unwrap_or_else(|| info.clone().sender.into_string());
        let dev_wallet = self.dev_wallet.may_load(deps.storage)?.unwrap_or_else(|| info.clone().sender.into_string());

        if sent_funds.clone() < qty.clone() as u128 * total_fee.clone() as u128 {
            return Err(ContractError::IncorrectFunds {});
        }

        let supply_limit = self.suply_limit.may_load(deps.storage)?.unwrap_or_else(|| 1000u64);
        let total_supply = self.total_supply.may_load(deps.storage)?.unwrap_or_else(|| 0u64);
        let mut real_purchase = cmp::min(qty.clone(), mint_per_tx.unwrap_or_else(|| 1u64));
        real_purchase = cmp::min(real_purchase.clone(), supply_limit - total_supply);
        let remainder = qty.clone() - real_purchase.clone();
        
        let mut msg = Response::new();
        let mint_response: Response<C> = self.mint(deps, info.clone(), format!("jarvis #{}", total_supply.clone()), qty.clone(), extension.clone())?;
        // msg.add_message(mint_response);
        let refund_amount = sent_funds.clone() - total_fee.clone() as u128 * remainder.clone() as u128;
        if refund_amount > 0 {
            let send_msg = BankMsg::Send {
                to_address: info.sender.into_string(),
                amount: vec![coin(refund_amount, "unibi")]
            };
        }
        let mint_fee_send = BankMsg::Send {
            to_address: withdraw_address.clone().to_string(),
            amount: vec![coin(mint_fee.clone() as u128 * real_purchase.clone() as u128, "unibi")]
        };
        let dev_fee_send = BankMsg::Send {
            to_address: dev_wallet.clone().to_string(),
            amount: vec![coin(refund_amount, "unibi")]
        };

        msg = msg.add_attribute("action", "buy");
        Ok(msg)
    }

    pub fn reserve(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        qty: &u64,
        extension: T
    ) -> Result<Response<C>, ContractError> {
        cw_ownable::assert_owner(deps.storage, &info.clone().sender)?;

        let mint_per_tx = self.mint_per_tx.may_load(deps.storage)?;

        let supply_limit = self.suply_limit.may_load(deps.storage)?.unwrap_or_else(|| 1000u64);
        let total_supply = self.total_supply.may_load(deps.storage)?.unwrap_or_else(|| 0u64);
        let mut reserved_amount = self.reserved_amount.may_load(deps.storage)?.unwrap_or_else(|| 0u64);
        let mut real_purchase = cmp::min(qty.clone(), mint_per_tx.unwrap_or_else(|| 1u64));
        real_purchase = cmp::min(real_purchase.clone(), supply_limit - total_supply);
        
        let mut msg = Response::new();
        let mint_response: Response<C> = self.mint(deps, info.clone(), "My NFT".to_string(), qty.clone(), extension.clone())?;
        // msg.add_message(mint_response);
        reserved_amount += real_purchase.clone();
        
        msg = msg
            .add_attribute("action", "reserve")
            .add_attribute("new_reserved", real_purchase.clone().to_string())
            .add_attribute("total_reserved_amount", reserved_amount.clone().to_string());
        Ok(msg)
    }

    pub fn toggle_sale_active(
        &self,
        deps: DepsMut,
        sender: &Addr,
    ) -> Result<Response<C>, ContractError> {
        cw_ownable::assert_owner(deps.storage, sender)?;

        let sale_active = self.sale_active.may_load(deps.storage)?;
        let new_sale_active = !sale_active.unwrap_or_else(|| false);
        self.sale_active.save(deps.storage, &new_sale_active)?;
        
        Ok(Response::new()
            .add_attribute("action", "toggle_sale_active")
            .add_attribute("sale_active", new_sale_active.to_string()))
    }
}

impl<'a, T, C, E, Q> Cw721Execute<T, C> for Cw721Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    type Err = ContractError;

    fn transfer_nft(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        recipient: String,
        token_id: String,
    ) -> Result<Response<C>, ContractError> {
        self._transfer_nft(deps, &env, &info, &recipient, &token_id)?;

        Ok(Response::new()
            .add_attribute("action", "transfer_nft")
            .add_attribute("sender", info.sender)
            .add_attribute("recipient", recipient)
            .add_attribute("token_id", token_id))
    }

    fn send_nft(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        contract: String,
        token_id: String,
        msg: Binary,
    ) -> Result<Response<C>, ContractError> {
        // Transfer token
        self._transfer_nft(deps, &env, &info, &contract, &token_id)?;

        let send = Cw721ReceiveMsg {
            sender: info.sender.to_string(),
            token_id: token_id.clone(),
            msg,
        };

        // Send message
        Ok(Response::new()
            .add_message(send.into_cosmos_msg(contract.clone())?)
            .add_attribute("action", "send_nft")
            .add_attribute("sender", info.sender)
            .add_attribute("recipient", contract)
            .add_attribute("token_id", token_id))
    }

    fn approve(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    ) -> Result<Response<C>, ContractError> {
        self._update_approvals(deps, &env, &info, &spender, &token_id, true, expires)?;

        Ok(Response::new()
            .add_attribute("action", "approve")
            .add_attribute("sender", info.sender)
            .add_attribute("spender", spender)
            .add_attribute("token_id", token_id))
    }

    fn revoke(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        spender: String,
        token_id: String,
    ) -> Result<Response<C>, ContractError> {
        self._update_approvals(deps, &env, &info, &spender, &token_id, false, None)?;

        Ok(Response::new()
            .add_attribute("action", "revoke")
            .add_attribute("sender", info.sender)
            .add_attribute("spender", spender)
            .add_attribute("token_id", token_id))
    }

    fn approve_all(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        operator: String,
        expires: Option<Expiration>,
    ) -> Result<Response<C>, ContractError> {
        // reject expired data as invalid
        let expires = expires.unwrap_or_default();
        if expires.is_expired(&env.block) {
            return Err(ContractError::Expired {});
        }

        // set the operator for us
        let operator_addr = deps.api.addr_validate(&operator)?;
        self.operators
            .save(deps.storage, (&info.sender, &operator_addr), &expires)?;

        Ok(Response::new()
            .add_attribute("action", "approve_all")
            .add_attribute("sender", info.sender)
            .add_attribute("operator", operator))
    }

    fn revoke_all(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        operator: String,
    ) -> Result<Response<C>, ContractError> {
        let operator_addr = deps.api.addr_validate(&operator)?;
        self.operators
            .remove(deps.storage, (&info.sender, &operator_addr));

        Ok(Response::new()
            .add_attribute("action", "revoke_all")
            .add_attribute("sender", info.sender)
            .add_attribute("operator", operator))
    }

    fn burn(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
    ) -> Result<Response<C>, ContractError> {
        let token = self.tokens.load(deps.storage, &token_id)?;
        self.check_can_send(deps.as_ref(), &env, &info, &token)?;

        self.tokens.remove(deps.storage, &token_id)?;
        self.decrement_tokens(deps.storage)?;

        Ok(Response::new()
            .add_attribute("action", "burn")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id))
    }
}

// helpers
impl<'a, T, C, E, Q> Cw721Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    pub fn _transfer_nft(
        &self,
        deps: DepsMut,
        env: &Env,
        info: &MessageInfo,
        recipient: &str,
        token_id: &str,
    ) -> Result<TokenInfo<T>, ContractError> {
        let mut token = self.tokens.load(deps.storage, token_id)?;
        // ensure we have permissions
        self.check_can_send(deps.as_ref(), env, info, &token)?;
        // set owner and remove existing approvals
        token.owner = deps.api.addr_validate(recipient)?;
        token.approvals = vec![];
        self.tokens.save(deps.storage, token_id, &token)?;
        Ok(token)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn _update_approvals(
        &self,
        deps: DepsMut,
        env: &Env,
        info: &MessageInfo,
        spender: &str,
        token_id: &str,
        // if add == false, remove. if add == true, remove then set with this expiration
        add: bool,
        expires: Option<Expiration>,
    ) -> Result<TokenInfo<T>, ContractError> {
        let mut token = self.tokens.load(deps.storage, token_id)?;
        // ensure we have permissions
        self.check_can_approve(deps.as_ref(), env, info, &token)?;

        // update the approval list (remove any for the same spender before adding)
        let spender_addr = deps.api.addr_validate(spender)?;
        token.approvals.retain(|apr| apr.spender != spender_addr);

        // only difference between approve and revoke
        if add {
            // reject expired data as invalid
            let expires = expires.unwrap_or_default();
            if expires.is_expired(&env.block) {
                return Err(ContractError::Expired {});
            }
            let approval = Approval {
                spender: spender_addr,
                expires,
            };
            token.approvals.push(approval);
        }

        self.tokens.save(deps.storage, token_id, &token)?;

        Ok(token)
    }

    /// returns true iff the sender can execute approve or reject on the contract
    pub fn check_can_approve(
        &self,
        deps: Deps,
        env: &Env,
        info: &MessageInfo,
        token: &TokenInfo<T>,
    ) -> Result<(), ContractError> {
        // owner can approve
        if token.owner == info.sender {
            return Ok(());
        }
        // operator can approve
        let op = self
            .operators
            .may_load(deps.storage, (&token.owner, &info.sender))?;
        match op {
            Some(ex) => {
                if ex.is_expired(&env.block) {
                    Err(ContractError::Ownership(OwnershipError::NotOwner))
                } else {
                    Ok(())
                }
            }
            None => Err(ContractError::Ownership(OwnershipError::NotOwner)),
        }
    }

    /// returns true if the sender can transfer ownership of the token
    pub fn check_can_send(
        &self,
        deps: Deps,
        env: &Env,
        info: &MessageInfo,
        token: &TokenInfo<T>,
    ) -> Result<(), ContractError> {
        // owner can send
        if token.owner == info.sender {
            return Ok(());
        }

        // any non-expired token approval can send
        if token
            .approvals
            .iter()
            .any(|apr| apr.spender == info.sender && !apr.is_expired(&env.block))
        {
            return Ok(());
        }

        // operator can send
        let op = self
            .operators
            .may_load(deps.storage, (&token.owner, &info.sender))?;
        match op {
            Some(ex) => {
                if ex.is_expired(&env.block) {
                    Err(ContractError::Ownership(OwnershipError::NotOwner))
                } else {
                    Ok(())
                }
            }
            None => Err(ContractError::Ownership(OwnershipError::NotOwner)),
        }
    }
}
