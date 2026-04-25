#!/bin/bash

cargo build --target wasm32-unknown-unknown 

cp target/wasm32-unknown-unknown/debug/rpg_2026.wasm web/ 

basic-http-server web/

# open -a "Google Chrome" "http://localhost:8080"