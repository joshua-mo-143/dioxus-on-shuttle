build:
    dx build --bin build-wasm --features web --release
    cp target/dx/build-wasm/debug/web/public public
