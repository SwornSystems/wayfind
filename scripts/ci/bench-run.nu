#!/usr/bin/env nix
#!nix develop .#ci --command nu

# Run the benchmarks.
def main []: nothing -> nothing {
    load-env {
        CARGO_PROFILE_DEV_CODEGEN_BACKEND: llvm
    }

    # NOTE: Must not be spawned as a child, so profiling hooks can follow it.
    exec cargo codspeed run --workspace
}
