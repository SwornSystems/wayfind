#!/usr/bin/env nix
#!nix develop .#ci --command nu

# Publish a release.
def main []: nothing -> nothing {
    if $env.CI? != "true" {
        print --stderr "Not running in CI"
        exit 1
    }

    let message: string = git log -1 --format=%s | str trim
    if $message !~ '^chore: Release v' {
        return
    }

    release-plz release
}
