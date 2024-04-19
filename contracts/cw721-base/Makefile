build:
	cargo wasm

optimize:
	docker run --rm -v "$$(pwd)":/code \
		--mount type=volume,source="$$(basename "$$(pwd)")_cache",target=/code/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		cosmwasm/rust-optimizer:0.14.0
test:
	cargo unit-test

upload-testnet:
	seid tx wasm store ./artifacts/raffle.wasm -y --from=dj --chain-id=atlantic-2 --node https://rpc.atlantic-2.seinetwork.io --gas=10000000 --fees=1000000usei --broadcast-mode=block

instantiate-testnet:
	seid tx wasm instantiate ${id} '{"count": 5, "owner": "sei1j7ah3st8qjr792qjwtnjmj65rqhpedjqf9dnsd"}' --chain-id atlantic-2 --from dj --gas=4000000 --fees=1000000usei --broadcast-mode=block --label raffle --no-admin --node https://rpc.atlantic-2.seinetwork.io

