.PHONY: check
check:
	cargo check

.PHONY: release
release:
	cargo build --release

.PHONY: example
example:
	cargo run -- --filter vary,cache https://www.fastly.com

.PHONY: example-json
example-json:
	cargo run -- --filter vary,cache https://www.fastly.com --json | jq
