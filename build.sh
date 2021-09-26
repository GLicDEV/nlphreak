#!/bin/bash

cargo build --target wasm32-unknown-unknown --package nlphreak --release
ic-cdk-optimizer target/wasm32-unknown-unknown/release/nlphreak.wasm -o target/wasm32-unknown-unknown/release/nlphreak_opt.wasm


#"build": "cargo build --target wasm32-unknown-unknown --package nlphreak --release",