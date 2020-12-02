# envoy-wasm-regional-routing
Example Envoy WebAssembly plugin using Rust

### How to run

The examples assumes working Rust and Cargo setup, running Docker daemon and Docker Compose installed.
Once you have the prerequisites you can build

```
make build
```

and then start the proxy

```
make run
```

You can test by sending requests to `localhost:10000`.

### References

1. https://github.com/proxy-wasm/proxy-wasm-rust-sdk
1. https://docs.eupraxia.io/docs/how-to-guides/deploy-rust-based-envoy-filter/
1. https://antweiss.com/blog/extending-envoy-with-wasm-and-rust/
1. https://medium.com/safetycultureengineering/edge-routing-with-envoy-and-lua-621f3d776c57
