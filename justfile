set default-list

fmt:
    cargo +nightly fmt 
license:
    cargo deny check licenses
check: fmt license
    cargo clippy

clippy:
    cargo watch -c -x clippy
