# Terra Send to Burn Address Smart Contract

*NOTE: Copied from [here](https://github.com/terra-project/cosmwasm-contracts) for build convenience, and with some added comments for educational purposes.*

## Build

```
$ cosm wasm
$ docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.10.3
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
