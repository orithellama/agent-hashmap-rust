#!/usr/bin/env bash
set -euo pipefail

echo "Running release checks..."
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
echo "Release checks passed"
