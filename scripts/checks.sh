#!/usr/bin/env -S nix develop .#ci --command bash
set -euxo pipefail
shopt -s globstar

nix flake check
nixfmt --check --width=120 **/*.nix

MAIN=$(git rev-parse --verify origin/main || git rev-parse --verify main)
if BASE=$(git merge-base "${MAIN}" HEAD) && [[ "${BASE}" != "$(git rev-parse HEAD)" ]]; then
  committed "${BASE}..HEAD"
fi
typos
tombi lint --error-on-warnings
zizmor --pedantic .github
cargo fmt --all --check
cargo shear --locked
cargo deny check --deny warnings
cargo clippy --locked --workspace --all-targets
cargo build --locked --workspace --all-targets
cargo nextest run --locked --workspace --no-tests pass
cargo test --locked --workspace --doc
cargo doc --locked --workspace --no-deps
