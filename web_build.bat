cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web --out-dir ./out/ --out-name "gps" ./target/wasm32-unknown-unknown/release/bevy_gps.wasm