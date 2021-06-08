# BSON Benchmark

A small webassembly benchmark designed to run in a browser to test the difference in performance when serializing and deserializing BSON compared to JSON.

## Requirements

* [Rust](https://www.rust-lang.org/)
* [wasm-pack](https://rustwasm.github.io/wasm-pack/)
* [basic-http-server](https://crates.io/crates/basic-http-server)
  * Or any other HTTP server of your choice.

## Running

Compile to a WASM module:
```sh
wasm-pack build --target no-modules --release
```

Execute the HTTP server:
```sh
basic-http-server
```

While the server is running open http://localhost:4000. The results will show on the browser's console, after freezing the tab for around 20 seconds.