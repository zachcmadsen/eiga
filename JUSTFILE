set dotenv-load

default:
    just --list

example name:
    cargo run --example {{name}}

doc:
    cargo doc --no-deps --open

it:
    cargo test --test it

test:
    cargo test
