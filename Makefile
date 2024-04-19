build:
	cargo wasm

optimize:
	docker run --rm -v "$$(pwd)":/code \
		--mount type=volume,source="$$(basename "$$(pwd)")_cache",target=/code/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		cosmwasm/rust-optimizer:0.14.0
test:
	cargo unit-test

WALLET=nibi10rdtquh3jl44hg00x0plzeawuclqqet0he4692
WALLET_NAME=wallet
CODE_ID=413
NFT_CONTRACT=nibi178kzznh9cepjckjefqc3mgt9gf9rfkyw6kk0pymeypx9rplggvyq9yjjuv


make-wallet:
	@nibid keys add wallet

show-wallet:
	@nibid keys show -a ${WALLET_NAME}

get-balance:
	@nibid query bank balances ${WALLET} --denom unibi

upload-testnet:
	@nibid tx wasm store artifacts/jarvis_airdrop.wasm --from ${WALLET} --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes

instantiate-testnet:
	@nibid tx wasm instantiate ${CODE_ID} '{"count": 1}' --admin ${WALLET} --label airdrop --from ${WALLET} --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes

exe_approve:
	$(eval exe_approve := $$(shell cat ./commands/exe_approve.json))
	@nibid tx wasm execute ${NFT_CONTRACT} '$(exe_approve)' --from ${WALLET} --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes 

exe_burn:
	$(eval exe_burn := $$(shell cat ./commands/exe_burn.json))
	@nibid tx wasm execute ${NFT_CONTRACT} '$(exe_burn)' --from ${WALLET} --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes 

exe_buy:
	$(eval exe_buy := $$(shell cat ./commands/exe_buy.json))
	@nibid tx wasm execute ${NFT_CONTRACT} '$(exe_buy)' --from ${WALLET} --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes 

exe_buy:
	$(eval exe_buy := $$(shell cat ./commands/exe_buy.json))
	@nibid tx wasm execute ${NFT_CONTRACT} '$(exe_buy)' --from ${WALLET} --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes 

exe_revoke_all:
	$(eval exe_revoke_all := $$(shell cat ./commands/exe_revoke_all.json))
	@nibid tx wasm execute ${NFT_CONTRACT} '$(exe_revoke_all)' --from ${WALLET} --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes 

exe_send_nft:
	$(eval exe_send_nft := $$(shell cat ./commands/exe_send_nft.json))
	@nibid tx wasm execute ${NFT_CONTRACT} '$(exe_send_nft)' --from ${WALLET} --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes 

exe_toggle_sale_active:
	$(eval exe_toggle_sale_active := $$(shell cat ./commands/exe_toggle_sale_active.json))
	@nibid tx wasm execute ${NFT_CONTRACT} '$(exe_toggle_sale_active)' --from ${WALLET} --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes 

exe_transfer_nft:
	$(eval exe_transfer_nft := $$(shell cat ./commands/exe_transfer_nft.json))
	@nibid tx wasm execute ${NFT_CONTRACT} '$(exe_transfer_nft)' --from ${WALLET} --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes 

set_dev_fee:
	$(eval set_dev_fee := $$(shell cat ./commands/set_dev_fee.json))
	@nibid tx wasm execute ${NFT_CONTRACT} '$(set_dev_fee)' --from ${WALLET} --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes 

set_mint_fee:
	$(eval set_mint_fee := $$(shell cat ./commands/set_mint_fee.json))
	@nibid tx wasm execute ${NFT_CONTRACT} '$(set_mint_fee)' --from ${WALLET} --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes 

set_mint_per_tx:
	$(eval set_mint_per_tx := $$(shell cat ./commands/set_mint_per_tx.json))
	@nibid tx wasm execute ${NFT_CONTRACT} '$(set_mint_per_tx)' --from ${WALLET} --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes 

set_name:
	$(eval set_name := $$(shell cat ./commands/set_name.json))
	@nibid tx wasm execute ${NFT_CONTRACT} '$(set_name)' --from ${WALLET} --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes 

set_owner:
	$(eval set_owner := $$(shell cat ./commands/set_owner.json))
	@nibid tx wasm execute ${NFT_CONTRACT} '$(set_owner)' --from ${WALLET} --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes 

set_sale_time:
	$(eval set_sale_time := $$(shell cat ./commands/set_sale_time.json))
	@nibid tx wasm execute ${NFT_CONTRACT} '$(set_sale_time)' --from ${WALLET} --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes 

set_supply_limit:
	$(eval set_supply_limit := $$(shell cat ./commands/set_supply_limit.json))
	@nibid tx wasm execute ${NFT_CONTRACT} '$(set_supply_limit)' --from ${WALLET} --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes 

set_symbol:
	$(eval set_symbol := $$(shell cat ./commands/set_symbol.json))
	@nibid tx wasm execute ${NFT_CONTRACT} '$(set_symbol)' --from ${WALLET} --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes 

set_withdraw_address:
	$(eval set_withdraw_address := $$(shell cat ./commands/set_withdraw_address.json))
	@nibid tx wasm execute ${NFT_CONTRACT} '$(set_withdraw_address)' --from ${WALLET} --gas auto --gas-adjustment 1.5 --gas-prices 0.025unibi --yes 

all_nft_info:
	$(eval all_nft_info := $$(shell cat ./commands/all_nft_info.json))
	@nibid query wasm contract-state smart ${NFT_CONTRACT} '$(all_nft_info)'

all_operators:
	$(eval all_operators := $$(shell cat ./commands/all_operators.json))
	@nibid query wasm contract-state smart ${NFT_CONTRACT} '$(all_operators)'

all_tokens:
	$(eval all_tokens := $$(shell cat ./commands/all_tokens.json))
	@nibid query wasm contract-state smart ${NFT_CONTRACT} '$(all_tokens)'

approval:
	$(eval approval := $$(shell cat ./commands/approval.json))
	@nibid query wasm contract-state smart ${NFT_CONTRACT} '$(approval)'

approve_all:
	$(eval approve_all := $$(shell cat ./commands/approve_all.json))
	@nibid query wasm contract-state smart ${NFT_CONTRACT} '$(approve_all)'

aprrovals:
	$(eval aprrovals := $$(shell cat ./commands/aprrovals.json))
	@nibid query wasm contract-state smart ${NFT_CONTRACT} '$(aprrovals)'

contract_info:
	$(eval contract_info := $$(shell cat ./commands/contract_info.json))
	@nibid query wasm contract-state smart ${NFT_CONTRACT} '$(contract_info)'

extension:
	$(eval extension := $$(shell cat ./commands/extension.json))
	@nibid query wasm contract-state smart ${NFT_CONTRACT} '$(extension)'

get_mint_fee:
	$(eval get_mint_fee := $$(shell cat ./commands/get_mint_fee.json))
	@nibid query wasm contract-state smart ${NFT_CONTRACT} '$(get_mint_fee)'

get_mint_per_tx:
	$(eval get_mint_per_tx := $$(shell cat ./commands/get_mint_per_tx.json))
	@nibid query wasm contract-state smart ${NFT_CONTRACT} '$(get_mint_per_tx)'

get_mint_price:
	$(eval get_mint_price := $$(shell cat ./commands/get_mint_price.json))
	@nibid query wasm contract-state smart ${NFT_CONTRACT} '$(get_mint_price)'

get_name:
	$(eval get_name := $$(shell cat ./commands/get_name.json))
	@nibid query wasm contract-state smart ${NFT_CONTRACT} '$(get_name)'

get_owner:
	$(eval get_owner := $$(shell cat ./commands/get_owner.json))
	@nibid query wasm contract-state smart ${NFT_CONTRACT} '$(get_owner)'

get_sale_time:
	$(eval GET_NFT_CONTRACT_ADDRESS := $$(shell cat ./commands/get_sale_time.json))
	@nibid query wasm contract-state smart ${NFT_CONTRACT} '$(get_sale_time)'

get_states:
	$(eval get_states := $$(shell cat ./commands/get_states.json))
	@nibid query wasm contract-state smart ${NFT_CONTRACT} '$(get_states)'

get_supply_limit:
	$(eval get_supply_limit := $$(shell cat ./commands/get_supply_limit.json))
	@nibid query wasm contract-state smart ${NFT_CONTRACT} '$(get_supply_limit)'

get_symbol:
	$(eval get_symbol := $$(shell cat ./commands/get_symbol.json))
	@nibid query wasm contract-state smart ${NFT_CONTRACT} '$(get_symbol)'

get_total_supply:
	$(eval get_total_supply := $$(shell cat ./commands/get_total_supply.json))
	@nibid query wasm contract-state smart ${NFT_CONTRACT} '$(get_total_supply)'

get_withdraw_address:
	$(eval get_withdraw_address := $$(shell cat ./commands/get_withdraw_address.json))
	@nibid query wasm contract-state smart ${NFT_CONTRACT} '$(get_withdraw_address)'

get_nft_contract_addres:
	$(eval GET_NFT_CONTRACT_ADDRESS := $$(shell cat ./commands/get_nft_contract_addr.json))
	@nibid query wasm contract-state smart ${NFT_CONTRACT} '$(GET_NFT_CONTRACT_ADDRESS)'

minter:
	$(eval minter := $$(shell cat ./commands/minter.json))
	@nibid query wasm contract-state smart ${NFT_CONTRACT} '$(minter)'

nft_info:
	$(eval nft_info := $$(shell cat ./commands/nft_info.json))
	@nibid query wasm contract-state smart ${NFT_CONTRACT} '$(nft_info)'

num_tokes:
	$(eval num_tokes := $$(shell cat ./commands/num_tokes.json))
	@nibid query wasm contract-state smart ${NFT_CONTRACT} '$(num_tokes)'

operator:
	$(eval operator := $$(shell cat ./commands/operator.json))
	@nibid query wasm contract-state smart ${NFT_CONTRACT} '$(operator)'

owner_of:
	$(eval owner_of := $$(shell cat ./commands/owner_of.json))
	@nibid query wasm contract-state smart ${NFT_CONTRACT} '$(owner_of)'

remove_withdraw_address:
	$(eval remove_withdraw_address := $$(shell cat ./commands/remove_withdraw_address.json))
	@nibid query wasm contract-state smart ${NFT_CONTRACT} '$(remove_withdraw_address)'

tokens:
	$(eval tokens := $$(shell cat ./commands/tokens.json))
	@nibid query wasm contract-state smart ${NFT_CONTRACT} '$(tokens)'

withdraw_funds:
	$(eval withdraw_funds := $$(shell cat ./commands/withdraw_funds.json))
	@nibid query wasm contract-state smart ${NFT_CONTRACT} '$(withdraw_funds)'
