# rust-wasm-game
Experimenting with making a game running under the wasm32-unknown-unknown target

## Running natively

Requires SDL2 development library.

```
cargo run --bin client-native --features native
```

## Web

Uses [wasm-bindgen](https://github.com/alexcrichton/wasm-bindgen) for generating javascript bindings.

### Setup

1. Install [yarn](https://yarnpkg.com/en/) (required by wasm-build for running webpack)
2. Set your default toolchain to nightly
```
rustup default nightly
```
3. Add the wasm32-unknown-unknown target
```
rustup target add wasm32-unknown-unknown
```
4. Install wasm-build
```
$ cargo install --git https://github.com/Healthire/wasm-build
```

### Building
```
$ wasm-build build --features web --bin client-web
```

The packaged application will be output to ./target/wasm-build/client-web/dist/client-web.js

### Running

```
$ wasm-build run --features web --bin client-web
```
