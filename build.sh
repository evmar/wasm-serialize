#!/bin/sh

set -e

cargo build --target wasm32-unknown-unknown --release -p demo
wasm-bindgen --out-dir pkg --typescript --target web --reference-types --omit-default-module-path \
    target/wasm32-unknown-unknown/release/demo.wasm
