# rust-wasm-game
Experimenting with making a game running under the wasm32-unknown-unknown target

## Running natively

Requires SDL2 development library.

```
cargo run --bin client-native --features native
```

## Web

Uses [wasm-bindgen](https://github.com/alexcrichton/wasm-bindgen) for generating javascript bindings.

### Building

The [wasm-build](https://github.com/Healthire/wasm-build) tool automates a bunch of steps required to build the game for web.

1. Install [yarn](https://yarnpkg.com/en/) (required by wasm-build for running webpack)
2. Add the wasm32-unknown-unkown target, install wasm-build, and build the project
```
$ rustup target add wasm32-unknown-unknown
$ cargo install --git https://github.com/Healthire/wasm-build
$ wasm-build --features web --bin client-web
```

wasm-build will ask to install the tools required for generating javascript bindings and packing up the project for web.

### Running

The packed build is found in `./target/wasm-build/client-web/dist` and must be served from an HTTP server. The npm [serve](https://www.npmjs.com/package/serve) application is useful for this.
```
$ serve ./target/wasm-build/client-web/dist
```
Then visit `http://localhost:5000/client-web.html` in your favorite webassembly-supporting web browser!
