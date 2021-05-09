# Terra Send to Burn Address Smart Contract

*NOTE: Copied from [here](https://github.com/terra-project/cosmwasm-contracts) for build convenience, and with some added comments for educational purposes.*

## Build

Build the target:

```
$ cargo wasm
```

Optimize the build:

```
$ docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.10.3
```

Store the contract on chain:

```
terracli tx wasm store artifacts/send_to_burn_address.wasm \
  --from test1 \
  --chain-id=localterra \
  --gas=auto \
  --fees=100000uluna \
  --broadcast-mode=block
```

Verify code on chain:

```
# code id may be different
$ terracli q wasm code 5 
codeid: 5
codehash: DtDf/gQ97DFmk/fjJiQQyqZ5xZREkDpC3i/o6s24T4U=
creator: terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8
```

Instantiate the contract:

```
$ terracli tx wasm instantiate 5 '{}' \
  --from test1 \
  --chain-id=localterra \
  --fees=10000uluna \
  --gas=auto \
  --broadcast-mode=block

height: 14434
txhash: 23FE032D1FF3291F70DC3E38C8541AF8ECB1BA6B7AEBE9870B6EB91C1B36B95F
codespace: ""
code: 0
data: ""
rawlog: '[{"msg_index":0,"log":"","events":[{"type":"instantiate_contract","attributes":[{"key":"owner","value":"terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8"},{"key":"code_id","value":"5"},{"key":"contract_address","value":"terra1plju286nnfj3z54wgcggd4enwaa9fgf5kgrgzl"}]},{"type":"message","attributes":[{"key":"action","value":"instantiate_contract"},{"key":"module","value":"wasm"},{"key":"sender","value":"terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8"}]}]}]'
logs:
- msgindex: 0
  log: ""
  events:
  - type: instantiate_contract
    attributes:
    - key: owner
      value: terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8
    - key: code_id
      value: "5"
    - key: contract_address
      value: terra1plju286nnfj3z54wgcggd4enwaa9fgf5kgrgzl
  - type: message
    attributes:
    - key: action
      value: instantiate_contract
    - key: module
      value: wasm
    - key: sender
      value: terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8
info: ""
gaswanted: 116682
gasused: 116101
tx: null
timestamp: ""
```

Send some coins to the smart contract:

```
terracli tx send \
  terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8 \
  terra1plju286nnfj3z54wgcggd4enwaa9fgf5kgrgzl \
  1000000uluna \
  --chain-id localterra \
  --fees=1000000uluna \
  --gas=auto \
  --broadcast-mode=block
```

Confirm coins sent:

```
$ terracli q account terra1plju286nnfj3z54wgcggd4enwaa9fgf5kgrgzl

  address: terra1plju286nnfj3z54wgcggd4enwaa9fgf5kgrgzl
  coins:
  - denom: uluna
    amount: "1000000"
  public_key: ""
  account_number: 24
  sequence: 0
```

Execute burn:

```
terracli tx wasm execute terra1plju286nnfj3z54wgcggd4enwaa9fgf5kgrgzl \
  '{"send_to_burn_account":{}}' \
  --from test1 \
  --chain-id localterra \
  --fees=1000000uluna \
  --gas=auto \
  --broadcast-mode=block
```

Query smart contract balance again:

```
$ terracli q account terra1plju286nnfj3z54wgcggd4enwaa9fgf5kgrgzl
|
  address: terra1plju286nnfj3z54wgcggd4enwaa9fgf5kgrgzl
  coins:
  - denom: uluna
    amount: "999000"
  public_key: ""
  account_number: 24
  sequence: 0
```

## Description

Send all funds to Burn Address(`terra1sk06e3dyexuq4shw77y3dsv480xv42mq73anxu`).

It has only one permissionless message handler.

```
{"send_to_burn_account": {}}
```

## New Version

- Tequila-0004: 

   code: 2152

   address: terra1950kmf2m7hxsn09waza0065lz859c0p63gmvaz

- Columbus-4:

   code: 156

   address: terra1vue77hd6ke8msuqhwv5tvsqr23nlftfukh0hx0

## Old Version

- Tequila-0004: 

   code: 1349
   
   address: terra1uxx0gr45wj0rpclgkrflf3vk8kvqsp3apvsdc3

- Columbus-4:

   code: 35
   
   address: terra1luk43x0g9vva7ws7ju9cl7g206wmeafzxl7vpz
