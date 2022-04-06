.PHONY: check
check:
	cargo check

.PHONY: test
test:
	cargo nextest run

.PHONY: release
release:
	cargo build --release

.PHONY: release-example
release-example:
	./target/release/crs --filter vary,cache https://www.fastly.com

.PHONY: example
example:
	cargo run -- --filter vary,cache https://www.fastly.com

.PHONY: example-json
example-json:
	cargo run -- --filter vary,cache https://www.fastly.com --json | jq
