#!/bin/sh

set -ex
cargo build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/cmdp.wasm ./ cmdp
