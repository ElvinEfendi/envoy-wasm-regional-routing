# envoy-wasm-regional-routing

Example Envoy WebAssembly plugin using Rust. This example serves two purposes

1. How to write an Envoy WASM plugin.
2. How to leverage Envoy's WASM support and implement dynamic routing.

For 1. you can also check the references below for more materials. For 2.,
I configured two regions `us_east1` and `us_central1`. Each cluster has
only single endpoint for the purpose of demonstration. Dynamic routing is implemented
in a way that given a request with HTTP Host, the plugin sends an async
HTTP request to a discovery service (`httpbin.org/anything` for this demo) and then
the discovery service returns a JSON response that instructs Envoy to which cluster request should go.
I mocked the discovery service so that for a HTTP Host where its first character's ASCII code
is odd it returns `us_central` and when it is even it returns `us_east1`.
Thus, `curl -I -H "Host: google.com" localhost:10000` would be routed to `us_central1`,
which means the request would be proxied to the only endpoint in that cluster that is `www.elvinefendi.com`.
On the other hand `curl -I -H "Host: doogle.com" localhost:10000` would go to `us_east1` cluster,
or `httpbin.org` endpoint.


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
