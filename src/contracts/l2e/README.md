# demo

cargo-near-new-project-description

## How to Build Locally?

Install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
RUSTFLAGS="-C link-arg=-s" rustup run stable cargo build --target wasm32-unknown-unknown --release
cargo near build
```

## How to Test Locally?

```bash
# Run Rust tests
cargo test --package l2e-top-near --lib --test test_basics -- --show-output
# Run js tests
npm run test
```

## How to Deploy?

Deployment is automated with GitHub Actions CI/CD pipeline.
To deploy manually, install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
near login
# Set testnetwork.testnet
# Set ft-l2e.testnet
# Set nft-l2e.testnet
near deploy testnetwork.testnet ./target/wasm32-unknown-unknown/release/l2e_top_near.wasm
near deploy ft-l2e.testnet ./tests/fungible_token.wasm
near deploy nft-l2e.testnet ./tests/non_fungible_token.wasm
```

## Initalizing the Contract

https://testnet.nearblocks.io/zh-cn/address/ft-l2e.testnet?tab=contract   new_default_meta  Write
https://testnet.nearblocks.io/zh-cn/address/nft-l2e.testnet?tab=contract  new_default_meta  Write

https://testnet.nearblocks.io/zh-cn/address/testnetwork.testnet?tab=contract init Write

## Useful Links

- [cargo-near](https://github.com/near/cargo-near) - NEAR smart contract development toolkit for Rust
- [near CLI](https://near.cli.rs) - Interact with NEAR blockchain from command line
- [NEAR Rust SDK Documentation](https://docs.near.org/sdk/rust/introduction)
- [NEAR Documentation](https://docs.near.org)
- [NEAR StackOverflow](https://stackoverflow.com/questions/tagged/nearprotocol)
- [NEAR Discord](https://near.chat)
- [NEAR Telegram Developers Community Group](https://t.me/neardev)
- NEAR DevHub: [Telegram](https://t.me/neardevhub), [Twitter](https://twitter.com/neardevhub)
