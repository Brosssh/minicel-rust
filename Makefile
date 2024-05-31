run:
	cargo run

build:
	cargo build

test:
	cargo test

watch:
	cargo watch -q -c -w src -w input.csv -x 'run -q'