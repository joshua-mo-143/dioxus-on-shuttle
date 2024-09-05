build:
    dx build --bin build-wasm --features web
    cp -r dist/public public
    rm -rf dist
