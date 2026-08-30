#!/usr/bin/env nix
#!nix develop .#ci --command nu

# Run the fuzzers.
def main [
    ...args: string, # Extra arguments for `libFuzzer`.
]: nothing -> nothing {
    load-env {
        CARGO_PROFILE_DEV_CODEGEN_BACKEND: llvm

        # NOTE: Must disable LTO.
        # https://github.com/rust-fuzz/cargo-fuzz/issues/384
        CARGO_PROFILE_RELEASE_LTO: "false"
    }

    rm --recursive --force fuzz/artifacts
    rm --recursive --force fuzz/corpus

    # NOTE: No `--locked` support.
    # https://github.com/rust-fuzz/cargo-fuzz/issues/312
    cargo fuzz build

    let list = do { cargo fuzz list } | complete
    if $list.exit_code != 0 {
        print --stderr "Failed to list the fuzz targets"
        exit 1
    }

    for target in ($list.stdout | lines) {
        (
            cargo fuzz run $target
                --
                -dict=fuzz/dict/wayfind.dict
                -timeout=0.0001 # 100 µs
                -max_total_time=30
                $"-fork=(sys cpu | length)"
                -print_final_stats=1
                ...$args
        )
    }
}
