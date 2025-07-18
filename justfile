feature_name := "unstable_deprecated"

default:
    just --list

run:
    cargo run

test *CARGO_FLAGS:
    cargo test --release {{CARGO_FLAGS}}

test-all *CARGO_FLAGS:
    cargo test --release --features={{feature_name}} {{CARGO_FLAGS}}

single-test TESTNAME *CARGO_FLAGS:
    cargo test --release --features={{feature_name}} {{TESTNAME}} {{CARGO_FLAGS}}

profile:
    cargo build --profile=profiling && samply record target/profiling/problem_of_13
