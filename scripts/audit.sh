#!/usr/bin/env bash
set -euo pipefail

if command -v cargo-audit >/dev/null 2>&1; then
  cargo audit
else
  echo "cargo-audit is not installed"
  echo "install: cargo install cargo-audit"
  exit 1
fi
