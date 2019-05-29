#!/bin/sh

cargo fmt --all
cargo clippy --all --all-targets --all-features -- -D warnings
cargo test --all
echo "start android build"
cd android
cargo build --target aarch64-linux-android --release --features "android"
cd ../ 
