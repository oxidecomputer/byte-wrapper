set positional-arguments := true

# Note: help messages should be 1 line long as required by just.

# Print a help message.
help:
    just --list

# Run `cargo hack --feature-powerset`.
powerset *args:
    NEXTEST_NO_TESTS=pass cargo hack --feature-powerset "$@"

# Build docs for the crate and direct dependencies.
rustdoc *args:
    RUSTC_BOOTSTRAP=1 RUSTDOCFLAGS='--cfg=doc_cfg' cargo doc --no-deps --all-features {{ args }}

# Generate README.md using `cargo-sync-rdme`.
generate-readmes:
    cargo sync-rdme --toolchain nightly-2025-11-05 --all-features

# Run cargo release in CI.
ci-cargo-release:
    # cargo-release requires a release off a branch.
    git checkout -B to-release
    cargo release publish --publish --execute --no-confirm -p serde_bytefmt
    git checkout -
    git branch -D to-release
