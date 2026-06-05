build:
	cargo build

erise:
	cargo run -- ./assets/earthrise.jpg

clean:
	rm -rf pact_*

tests:
	cargo test
