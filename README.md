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

1. Install [yarn](https://yarnpkg.com/en/) (required by wasm-bin for running webpack)
2. Set your default toolchain to nightly
```
rustup default nightly
```
3. Add the wasm32-unknown-unknown target
```
rustup target add wasm32-unknown-unknown
```
4. Install wasm-bin
```
$ cargo install --git https://github.com/Healthire/wasm-bin
```

### Building
```
$ wasm-bin build --features web --bin client-web
```

The packaged application will be output to ./target/wasm-bin/client-web/dist/client-web.js

### Running

```
$ wasm-bin run --features web --bin client-web
```
