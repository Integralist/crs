# Compilation check
.PHONY: check
check:
	cargo check

# Test suite
.PHONY: test
test:
	cargo nextest run

# Release build
.PHONY: release
release:
	cargo build --release

# Validate release build
.PHONY: release-example
release-example:
	./target/release/crs --filter vary,cache https://www.fastly.com

# Validate runtime code
.PHONY: run-example
run-example:
	cargo run -- --filter vary,cache https://www.fastly.com

# Validate runtime code with JSON output
.PHONY: run-example-json
run-example-json:
	cargo run -- --filter vary,cache https://www.fastly.com --json | jq

# Validate runtime code with expected failure
.PHONY: run-example-failure
run-example-failure:
	cargo run -- --filter vary,cache https://www.fastly.com/does-not-exist
