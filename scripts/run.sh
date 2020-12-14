#!/bin/sh

run_dev() {
    wasm-pack build --target web --dev
    basic-http-server ./pkg -a 0.0.0.0:4001
}

wasm-pack build --debug -t web -d public/build/ --dev
basic-http-server ./public -a 0.0.0.0:5001
