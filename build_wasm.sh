#!/bin/bash
rm -rf ./out/
mkdir ./out
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/thetawave.wasm
cp -R ./wasm-page-template/* ./out/
cp -R ./assets ./out/