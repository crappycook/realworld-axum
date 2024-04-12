fmt:
	cargo fmt --all --check
test:
	cargo test
run-rest:
	cargo run --bin rest-server

.PHONY: fmt test run-rest