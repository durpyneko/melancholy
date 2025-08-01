@echo off
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --target web --out-dir dist --no-typescript target\wasm32-unknown-unknown\release\melancholy.wasm