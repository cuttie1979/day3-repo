#!/usr/bin/env bash
# The whole demo in one command, in the order it is meant to be shown.
# Use it as the fallback if anything on the day misbehaves: it is the same four steps, unedited.

set -u
cd "$(dirname "$0")"
run() { echo; echo "===================== $* ====================="; "$@"; }

run cargo test --quiet --test roundtrip
run cargo test --quiet --test properties
run cargo test --quiet --features hardened --test roundtrip
run cargo test --quiet --features hardened --test properties
run cargo run --quiet --example key_debug
run cargo run --quiet --features hardened --example key_debug
run ./mutate.sh

echo
echo "===================== the diff, in full ====================="
diff -u src/naive.rs src/hardened.rs || true
