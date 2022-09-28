set dotenv-load

default:
    just --list

build: clippy
    cargo build

check: fmt clippy build doc test

clippy:
    cargo clippy

doc:
    cargo doc --no-deps

ex name:
    cargo run --example {{name}}

fmt:
    cargo fmt

it:
    cargo test --test it

package: check
    cargo package

test:
    cargo test
