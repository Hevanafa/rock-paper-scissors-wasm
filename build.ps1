cargo build --release --target wasm32-unknown-unknown
wasm-bindgen .\target\wasm32-unknown-unknown\release\rock_paper_scissors_wasm.wasm --out-dir pkg --target=web
