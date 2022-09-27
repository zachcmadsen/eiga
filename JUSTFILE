set dotenv-load

default:
    just --list

build: clippy
    cargo build

clippy:
    cargo clippy

doc:
    cargo doc --no-deps

example name:
    cargo run --example {{name}}

fmt:
    cargo fmt

it:
    cargo test --test it

package: fmt clippy build doc test
    cargo package

test:
    cargo test
