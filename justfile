set default-list

fmt:
    cargo +nightly fmt 
license:
    cargo deny check licenses
check: fmt license
    cargo clippy

clippy:
    cargo watch -c -x clippy

commit_types := "add fix docs chore style refactor revert"
[positional-arguments]
commit type +message:
    #!/usr/bin/env bash
    set -euo pipefail
    type="$1"; shift
    grep -qw -- "$type" <<< "{{ commit_types }}" || { echo "invalid type: $type" >&2; exit 1; }
    git commit -S -m "$type: $*"
