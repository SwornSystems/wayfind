#!/usr/bin/env -S nix develop .#ci --command nu

# Build the documentation.
def main []: nothing -> nothing {
    cargo doc --locked --workspace --no-deps --document-private-items
    rm target/doc/.lock
    '<meta http-equiv="refresh" content="0; url=wayfind/index.html">' | save --force target/doc/index.html
}
