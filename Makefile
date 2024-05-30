run:
	cargo run

build:
	cargo build

watch:
	cargo watch -q -c -w src -w input.csv -x 'run -q'