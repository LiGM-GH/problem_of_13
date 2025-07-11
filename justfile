default:
    just --list

run:
    cargo run

test:
    cargo test

test-all:
    cargo test --features=multithreaded

single-test TESTNAME:
    cargo test --features=multithreaded {{TESTNAME}}
