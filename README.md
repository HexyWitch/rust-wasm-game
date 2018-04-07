# rust-wasm-game
Experimenting with making a game running under the wasm32-unknown-unknown target

## Running server

```
cargo run --features server --bin server
```

## Running native client

Requires SDL2 development library.

```
cargo run --bin client
```

## Running web client

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
$ wasm-bin build --bin client
```

The packaged application will be output to ./target/wasm-bin/client/dist/client.js

### Running

```
$ wasm-bin run --bin client
```
