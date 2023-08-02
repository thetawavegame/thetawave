#!/usr/bin/env bash
set -euo pipefail
rm -rf ./out/
mkdir ./out
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
cargo install wasm-bindgen-cli
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/thetawave.wasm
cp -R ./wasm-page-template/* ./out/
cp -R ./assets ./out/
