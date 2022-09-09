set dotenv-load

default:
    just --list

example name:
    cargo run --example {{name}}

doc:
    cargo doc --no-deps --open

test:
    cargo test --test it
