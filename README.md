# Plasma Rust Framework

[Draft] Plasma Chamber's Rust implemenation.
Plasma Chamber is now a compliant of Plasma Core.
This repositry must be conpatible with [plasma-core](https://github.com/plasma-group/plasma-core) and [pigi](https://github.com/plasma-group/pigi)

[![Build Status](https://travis-ci.org/cryptoeconomicslab/plasma-rust-framework.svg?branch=master)](https://travis-ci.org/cryptoeconomicslab/plasma-rust-framework)

## Overview

- Gradually try shifting from JS to Rust
- For browser and NodeJS: Publish WASM to WAPM, and publish generated wrapper & @types to NPM
- For App: Compile rust to each target environment

## Build Source Code

```
cargo build --release
```

### Run Manually

```
./target/release/plasma-chamber
```

# Contributing

## Contributing Guide

TBD

## Test

```
cargo test --all
```

## Lint

We are using [rustfmt](https://github.com/rust-lang/rustfmt) and [clippy](https://github.com/rust-lang/rust-clippy) to check code style.

```
cargo fmt --all -- --check
```

```
cargo clippy --all --all-targets --all-features -- -D warnings
```

### fixing

```
cargo fmt --all
```
