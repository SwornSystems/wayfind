#!/usr/bin/env nix
#!nix develop .#ci --command nu

# Run the benchmarks.
def main []: nothing -> nothing {
    load-env {
        CARGO_PROFILE_DEV_CODEGEN_BACKEND: llvm
    }

    cargo codspeed run --workspace
}
