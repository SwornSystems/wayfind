#!/usr/bin/env nix
#!nix develop .#ci --command nu

# Run all linters and formatters.
def main []: nothing -> nothing {
    let markdown: list<string> = files "*.md"
    let scripts: list<string> = files "*.nu"
    let nix: list<string> = files "*.nix"

    # Git
    committed origin/main..HEAD

    # GitHub
    zizmor --pedantic .github

    # Spellchecking
    typos

    # Markdown
    lychee --verbose .

    let alerts = vale --no-exit --output=JSON ...$markdown | from json
    if ($alerts | is-not-empty) {
        vale ...$markdown
        exit 1
    }

    # TOML
    tombi lint --error-on-warnings

    # Nushell
    nufmt --dry-run ...$scripts
    nu-lint --config .nu-lint.toml ...$scripts

    # Nix
    nix flake check
    nixfmt --check --width=120 ...$nix
    deadnix --fail .

    # Rust
    cargo fmt --all --check
    cargo shear --locked
    cargo deny check --deny warnings
    cargo clippy --locked --workspace --all-targets
    cargo build --locked --workspace --all-targets
    cargo nextest run --locked --workspace --no-tests pass
    cargo test --locked --workspace --doc
    cargo doc --locked --workspace --no-deps

    # Documentation
    rm --recursive --force target/doc
    with-env {
        RUSTDOCFLAGS: "-Z unstable-options --output-format json"
    } {
        cargo doc --locked --workspace --no-deps --document-private-items
    }

    (
        glob "target/doc/*.json"
        | each { open $in | get index | values }
        | flatten
        | where crate_id == 0 and docs != null
        | get docs
        | str join "\n"
        | save --force target/doc/prose.md
    )

    let docs = vale --no-exit --output=JSON target/doc/prose.md | from json
    if ($docs | is-not-empty) {
        vale target/doc/prose.md
        exit 1
    }
}

def files [pattern: string]: nothing -> list<string> {
    git ls-files --cached --others --exclude-standard $pattern | lines
}
