# rust-wasm-game
Experimenting with making a game running under the wasm32-unknown-unknown target

## Running natively

Requires SDL2 development library.

```
cargo run --bin client-native --features native
```

## Running for web

Currently requires cargo-web for exporting the main function and functions table in wasm. Installation instructions can be found at:  https://github.com/koute/cargo-web

The only currently supported target is wasm32-unknown-unknown, and requires the javascript runtime found in the /static/ folder.

To run start the client-web binary using cargo-web.

```
$ cargo web start --bin client-web --features web --target=wasm32-unknown-unknown
```
