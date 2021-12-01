<p align="center">
  <a href="https://github.com/interlay/interbtc">
    <img alt="interBTC" src="/docs/img/banner.jpg">
  </a>
  <h2 align="center">interBTC</h2>

  <p align="center">
    A trust-minimized bridge from Bitcoin to Polkadot.
    <br />
    <a href="https://spec.interlay.io/"><strong>Explore the specification »</strong></a>
    <br />
    <br />
    <a href="https://github.com/interlay/interbtc/issues">Report Bug</a>
    ·
    <a href="https://github.com/interlay/interbtc/issues">Request Feature</a>
  </p>
</p>

This repository is hosted on GitHub: [https://github.com/interlay/interbtc](https://github.com/interlay/interbtc) with a mirror on [GitLab](https://gitlab.com/interlay/btc-parachain) and [radicle](rad:git:hnrkxrw3axafn8n5fwo8pspjgtbt6jj6qe6mo).

## Table of Contents

- [About the Project](#about-the-project)
  - [Built With](#built-with)
- [Roadmap](#roadmap)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)
- [Acknowledgements](#acknowledgements)

## About the Project

The interBTC runtime allows the creation of **interBTC**, a fungible token that represents Bitcoin in the Polkadot ecosystem.
Each interBTC is backed by Bitcoin 1:1 and allows redeeming of the equivalent amount of Bitcoins by relying on a collateralized third-party.

The project uses the concept of [Cryptocurrency-backed Assets](https://xclaim.io) to lock Bitcoin on the Bitcoin blockchain and issue BTC-backed tokens on the BTC Parachain.
The implementation is based on the [interBTC specification](https://spec.interlay.io/).

### Built with

The interBTC project is built with:

- [Rust](https://www.rust-lang.org/)
- [Substrate](https://substrate.dev/)

    <img src="https://spec.interlay.io/_images/overview.png" alt="Logo" width="500">

### Development Progess

The Substrate runtime makes use of various custom pallets that are found in the [crates](./crates) folder.

- [bitcoin](crates/bitcoin): Library for Bitcoin type, parsing and verification functions.
- [btc-relay](crates/btc-relay): Stateful SPV client for Bitcoin. Stores Bitcoin main chain, tracks forks, verifies Merkle proofs and validates specific transaction formats.
- [currency](crates/currency) Handles currencies used as backing collateral (e.g. DOT/KSM) and issued tokens (e.g. interBTC).
- [oracle](crates/oracle): Trusted providers use this to set exchange rates and Bitcoin fee estimates.
- [fee](crates/fee): Participant reward calculation and distribution.
- [issue](crates/issue): Handles issuing of interBTC for BTC on Bitcoin.
- [redeem](crates/redeem) Handles redeeming of interBTC for BTC on Bitcoin.
- [refund](crates/refund) Handles refunds for when a vault receives more BTC than it can cover.
- [replace](crates/replace) Handles replacing vaults.
- [security](crates/security): Handles BTC Parachain status and error changes.
- [relay](crates/relay): Handles block submission and theft reporting.
- [vault-registry](crates/vault-registry): Handles registration, collateral and liquidation of vaults.

## Getting started

### Prerequisites

```
curl https://sh.rustup.rs -sSf | sh
```

Please also install the following dependencies:

- `cmake`
- `clang` (>=10.0.0)
- `clang-dev`
- `libc6-dev`
- `libssl-dev`

### Installation

Building requires a specific rust toolchain and nightly compiler version. The
requirements are specified in the [./rust-toolchain.toml](./rust-toolchain.toml)
[override file][].

Running `rustup show` from the root directory of this repo should be enough to
set up the toolchain and you can inspect the output to verify that it matches
the version specified in the override file.

To build, run:

```
cargo build
```

For more detailed development instructions [see here](./docs/README.md).

[override file]: https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file

### Testing

```
cargo test
```

To run with coverage, using [cargo-cov](https://github.com/kennytm/cov):

```
cargo install cargo-cov

# clean up previous coverage result
cargo cov clean

# test the code
cargo cov test

# open the coverage report
cargo cov report --open
```

### Running - Standalone

To run a local development node, use the `dev` chain spec.

```shell
cargo run --release --bin interbtc-standalone -- --dev
```

Clear the database using the `purge-chain` command.

```shell
cargo run --release --bin interbtc-standalone -- purge-chain --dev
```

Additional CLI usage options are available and may be shown by running `cargo run --bin interbtc-standalone -- --help`.

### Running - Parachain

To run a local development node, use the `dev` chain spec.

```shell
cargo run --release --bin interbtc-parachain -- --dev
```

To connect with a local relay-chain follow [these instructions](docs/rococo.md).

#### Test Coverage

Test coverage reports available under [docs/testcoverage.html](https://github.com/interlay/interbtc/blob/master/docs/testcoverage.html)

### Substrate Chain Configuration

The Substrate runtime configuration is in the [parachain](./parachain) folder.

### Javascript / Typescript

When interacting via polkadot{.js} you will need to use our [custom types](https://github.com/interlay/interbtc-types). Please also checkout [interbtc-js](https://github.com/interlay/interbtc-js) for a more complete (strongly-typed) library.

## Contributing

If you would like to contribute, please file an issue on GitHub or reach out to us.

- [Discord](https://discord.gg/C8tjMbgVXh)
- [Telegram](https://t.me/joinchat/G9FaYhNbJK9v-6DN3IyhJw)

We are [hiring](https://interlay.recruitee.com/)!

## License

(C) Copyright 2021 [Interlay](https://www.interlay.io) Ltd

interBTC is currently licensed under the terms of the Apache License (Version 2.0). See LICENSE

## Contact

Website: [Interlay.io](https://www.interlay.io)

Twitter: [@interlayHQ](https://twitter.com/InterlayHQ)

Email: contact@interlay.io

## Acknowledgements

This project is supported by a [Web3 Foundation grant](https://web3.foundation/grants/).

We would also like to thank the following teams for their continuous support:

- [Parity Technologies](https://www.parity.io/)

<p align="center">
  <a href="https://web3.foundation/grants/">
    <img src="/docs/img/web3GrantsBadge.png">
  </a>
</p>
