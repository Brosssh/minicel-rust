run:
	cargo run

build:
	cargo build

watch:
	cargo watch -q -c -w src -x 'run -q'