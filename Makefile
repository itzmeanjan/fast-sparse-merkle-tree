default: fmt clippy test check

test:
	cargo test --profile test-release --all --all-features

clippy:
	cargo clippy  --all --all-features --all-targets

fmt:
	cargo fmt --all -- --check

check:
	cargo check --no-default-features

clean:
	cargo clean

bench:
	RUSTFLAGS="-C target-cpu=native" cargo bench --profile optimized --bench smt_benchmark
