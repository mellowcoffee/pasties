default:
    @just --list
fmt:
    cargo +nightly fmt 
test:
    cargo run -- config.example.toml
