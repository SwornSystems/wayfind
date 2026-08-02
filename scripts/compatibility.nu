#!/usr/bin/env nix
#!nix develop .#ci-compatibility --command nu

# Build for the compatibility targets.
def main []: nothing -> nothing {
    hide-env --ignore-errors CARGO_PROFILE_DEV_CODEGEN_BACKEND

    cargo build --locked --lib --package wayfind
    cargo build --locked --lib --package wayfind --target thumbv6m-none-eabi
    cargo build --locked --lib --package wayfind --target wasm32-unknown-unknown
}
