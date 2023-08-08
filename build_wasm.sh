#!/usr/bin/env bash
set -euo pipefail
rm -rf ./out/
mkdir ./out
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
cargo install wasm-bindgen-cli
wasm-bindgen --out-dir ./out/ --target web --split-linked-modules ./target/wasm32-unknown-unknown/release/thetawave.wasm
wasm-opt out/thetawave_bg.wasm -Oz -o thetawave_bg.wasm
cp -R ./wasm-page-template/* ./assets/ ./thetawave_bg.wasm ./netlify.toml ./out/
cp ./assets/texture/captain_character.png out/favicon.png
