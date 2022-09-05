set dotenv-load

default:
    just --list

test:
    cargo test --test it

example name:
    cargo run --example {{name}}
