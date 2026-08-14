set default-list

fmt:
    cargo +nightly fmt 
license:
    cargo deny check licenses
check: fmt license
    cargo clippy

watch:
    cargo watch -c -x clippy
dev config="config.example.toml":
    cargo watch -c -x "run -- {{config}}"
