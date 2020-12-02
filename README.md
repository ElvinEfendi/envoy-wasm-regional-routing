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
