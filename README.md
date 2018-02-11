# rust-wasm-game
Experimenting with making a game running under the wasm32-unknown-unknown target

## Running

Currently requires https://github.com/koute/cargo-web for exposing the main function and function table to the web.

```
$ cargo install cargo-web
$ cargo web start --target=wasm32-unknown-unknown
```
