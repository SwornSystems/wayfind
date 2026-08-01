#!/usr/bin/env -S nix develop .#ci --command nu

# Build the benchmarks.
def main []: nothing -> nothing {
    load-env {
        CARGO_PROFILE_DEV_CODEGEN_BACKEND: llvm
    }

    cargo codspeed build --locked --workspace
}
