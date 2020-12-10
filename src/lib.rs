#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use log::info;
use std::time::Duration;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

#[derive(Deserialize)] struct HttpBinResponse { args: HttpBinResponseArgs }
#[derive(Deserialize)] struct HttpBinResponseArgs { region: String }

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_http_context(|_, _| -> Box<dyn HttpContext> {
        Box::new(RegionalRouter)
    });
}

struct RegionalRouter;

impl Context for RegionalRouter {
    fn on_http_call_response(&mut self, _: u32, _: usize, body_size: usize, _: usize) {
        if let Some(body) = self.get_http_call_response_body(0, body_size) {
            let data: HttpBinResponse = serde_json::from_slice(body.as_slice()).unwrap();
            
            info!("Routing to the region: {}", data.args.region);

            // NOTE: in the envoy.yaml we configured the route with `cluster_header: region`,
            // which means it will pick the cluster from the value of `region` HTTP header.
            // So here we get the region from our service discovery for given host, and set the
            // region header based on that. The rest should be taken care by Envoy.
            self.set_http_request_header(&"region", Some(&data.args.region));

            // NOTE: for now routing table is not flushed automatically,
            // this is a workaround. See https://github.com/proxy-wasm/spec/issues/16.
            self.clear_http_route_cache();
            
            self.resume_http_request();
            
            return
        }
    }
}

impl HttpContext for RegionalRouter {
    fn on_http_request_headers(&mut self, _: usize) -> Action {
        // NOTE: normally this or similar logic would be part of 
        // the discovery service, but we are merely mocking using httpbin.org.
        let host = self.get_http_request_header(":authority").unwrap();
        let hash_code = (host.chars().next().unwrap() as u32) % 2;
        let mut expected_region = "us_east1";
        info!("Hash code of {} is {}", host, hash_code);
        if hash_code == 1 {
            expected_region = "us_central1";
        }

        self.dispatch_http_call(
            "discovery_service",
            vec![
                (":method", "GET"),
                (":path", &format!("/anything?region={}", expected_region)),
                (":authority", "httpbin.org"),
            ],
            None,
            vec![],
            Duration::from_secs(5),
        ).unwrap();

        Action::Pause
    }

    fn on_http_response_headers(&mut self, _: usize) -> Action {
        self.set_http_response_header("Processed-By", Some(&"Regional Router"));
        
        Action::Continue
    }
}
