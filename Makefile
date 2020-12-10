.PHONY: run
run:
	docker-compose up --build

build:
	cargo +nightly build --target=wasm32-unknown-unknown --release

.PHONY: clean
clean:
	rm -r target || true

.PHONY: test
	# hash code is not 1, so this gets routed to 'us_east1', which has httpbin.org as origin/endpoint
	curl -I -H "Host: httpbin.org" localhost:10000

	# hash code is 1, so this gets routed to 'us_central1', which has www.elvinefendi.com as origin/endpoint
	curl -I -H "Host: www.elvinefendi.com" localhost:10000

	# hash code is 1, so this gets routed to 'us_central1', which has httpbin.org as origin/endpoint
	# this returns 403, because cloudflare drops requests if they don't match expected host header (google.com != elvinefendi.com)
	curl -I -H "Host: google.com" localhost:10000

	# hash code is not 1, so this gets routed to 'us_east1', which has httpbin.org as origin/endpoint
	# this works because httpbin.org origin does not care about Host header
	curl -I -H "Host: doogle.com" localhost:10000
