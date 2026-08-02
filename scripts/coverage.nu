#!/usr/bin/env nix
#!nix develop .#ci --command nu

# Generate a coverage report.
def main []: nothing -> nothing {
    load-env {
        CARGO_PROFILE_DEV_CODEGEN_BACKEND: llvm
    }

    cargo llvm-cov --no-report nextest --locked --workspace
    cargo llvm-cov --no-report --doc --locked --workspace
    cargo llvm-cov report --doctests --codecov --output-path codecov.json
}
