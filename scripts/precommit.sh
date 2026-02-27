#!/usr/bin/env bash
# SPDX-License-Identifier: MIT

set -euo pipefail

echo "[fmt]"
cargo fmt --all -- --check

echo "[clippy]"
cargo clippy --all-targets -- -D warnings

echo "[test]"
cargo test --all --locked

echo "[idl: examples]"
for f in examples/*.idl; do
  echo "Check $f"
  cargo run --quiet --bin hddsgen -- check "$f"
done

echo "[idl: canonical fmt strict]"
for f in examples/canonical/*.idl; do
  tmp=$(mktemp)
  cargo run --quiet --bin hddsgen -- fmt "$f" > "$tmp"
  diff -u "$f" "$tmp"
  rm -f "$tmp"
done

echo "OK"
