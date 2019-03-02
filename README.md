# rust-wasm-game
Experimenting with making a game running under the wasm32-unknown-unknown target

## Running native client

Requires that you have the SDL2 development library installed on your computer.

```
cargo run
```

## Running web client

Uses [wasm-bindgen](https://github.com/alexcrichton/wasm-bindgen) for generating javascript bindings.

### Setup

1. Set your default toolchain to nightly
```
rustup default nightly
```
2. Add the wasm32-unknown-unknown target
```
rustup target add wasm32-unknown-unknown
```
3. Install wasm-bin
```
$ cargo install --git https://github.com/Healthire/wasm-bin
```

### Building
```
$ wasm-bin build
```

The packaged application will be output to ./target/wasm-bin/game

### Running

```
$ wasm-bin run
```
