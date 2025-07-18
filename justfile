feature_name := "unstable_deprecated"

default:
    just --list

run:
    cargo run --release --features={{feature_name}}

test *CARGO_FLAGS:
    cargo test --workspace --release {{CARGO_FLAGS}}

test-all *CARGO_FLAGS:
    cargo test --workspace --release --features={{feature_name}} {{CARGO_FLAGS}}

single-test TESTNAME *CARGO_FLAGS:
    cargo test --workspace --release --features={{feature_name}} {{TESTNAME}} {{CARGO_FLAGS}}

profile:
    cargo build --profile=profiling && samply record target/profiling/problem_of_13

check:
    cargo check --workspace --features={{feature_name}}
