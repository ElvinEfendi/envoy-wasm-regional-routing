run:
	docker-compose up --build

build:
	cargo +nightly build --target=wasm32-unknown-unknown --release

.PHONY: clean
clean:
	rm -r target || true
