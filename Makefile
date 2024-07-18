TEST_STACK_SIZE=104857600

test:
	RUST_MIN_STACK=$(TEST_STACK_SIZE) cargo test
fmt:
	cargo fmt --all
lint:
	cargo clippy -- -D warnings
check: fmt lint test
