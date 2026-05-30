build:
	cargo build

erise:
	cargo run -- ./assets/earthrise.jpg

clean:
	rm -rf earthrise/
	rm -rf kanagawa/
	rm -rf sky/
	rm -rf bluemarble/

tests:
	cargo test
