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

.PHONY: run-example
run-example:
	cargo run -- --filter vary,cache https://www.fastly.com

.PHONY: run-example-json
run-example-json:
	cargo run -- --filter vary,cache https://www.fastly.com --json | jq

.PHONY: run-example-failure
run-example-failure:
	cargo run -- --filter vary,cache https://www.fastly.com/does-not-exist
