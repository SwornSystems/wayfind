#!/usr/bin/env nix
#!nix develop --command nu

# Open a release pull request.
def main []: nothing -> nothing {
    let auth = do { gh auth token } | complete
    if $auth.exit_code != 0 {
        print --stderr "Not signed in to GitHub"
        exit 1
    }

    load-env {
        GIT_TOKEN: ($auth.stdout | str trim)
    }

    release-plz release-pr
}
