.DEFAULT_GOAL := release

TEST_URL ?= "https://httpbin.org/anything"

# Compilation check
.PHONY: check
check: ## Validate app can compile
	cargo check

# Install binary
.PHONY: install
install: release ## Install binary
	mv ./target/release/crs /usr/local/bin/crs

# Test suite
.PHONY: test
test: ## Run test suite
	cargo nextest run

# Release build
.PHONY: release
release: ## Generate release binary
	cargo build --release

# Validate release build
.PHONY: release-example
release-example: ## Run release binary
	./target/release/crs --filter server,content $(TEST_URL)

# Run application code
.PHONY: run-example
run: ## Run app with custom args (e.g. ARGS=--help)
	cargo run -- $(ARGS)

# Validate runtime code
.PHONY: run-example
run-example: ## Run app using TEST_URL + include response body
	cargo run -- --filter server,content --body $(TEST_URL)

# Validate runtime code with JSON output
.PHONY: run-example-json
run-example-json: ## Run app and expect JSON output
	cargo run -- --filter server,content $(TEST_URL) --json | jq

# Validate runtime code with expected failure
.PHONY: run-example-failure
run-example-failure:  ## Run app but expect error output
	cargo run -- --filter server,content https://httpbin.org/status/404

.PHONY: help
help:
	@printf "Targets\n"
	@(grep -h -E '^[0-9a-zA-Z_.-]+:.*?## .*$$' $(MAKEFILE_LIST) || true) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-22s\033[0m %s\n", $$1, $$2}'
	@printf "\nDefault target\n"
	@printf "\033[36m%s\033[0m" $(.DEFAULT_GOAL)
	@printf "\n\nMake Variables\n"
	@(grep -h -E '^[0-9a-zA-Z_.-]+\s[:?]?=.*? ## .*$$' $(MAKEFILE_LIST) || true) | sort | awk 'BEGIN {FS = "[:?]?=.*?## "}; {printf "\033[36m%-25s\033[0m %s\n", $$1, $$2}'

.PHONY: all clean
