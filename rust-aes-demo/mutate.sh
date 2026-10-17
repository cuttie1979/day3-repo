#!/usr/bin/env bash
# Seed one fault in the hardened implementation and see whether the tests catch it.
#
# The fault: the nonce becomes a constant, so the same plaintext under the same key produces the
# same ciphertext again. Nothing else changes. This is the round we do not skip: a check we have
# never seen fail is not a check, it is a hope.
#
# The file is restored afterwards, whatever happens.

set -u
cd "$(dirname "$0")"

SRC=src/hardened.rs
BAK=$(mktemp)
cp "$SRC" "$BAK"
restore() { cp "$BAK" "$SRC"; rm -f "$BAK"; }
trap restore EXIT

echo "=== the fault: the nonce is no longer random ==="
python3 - "$SRC" <<'PY'
import sys, pathlib
p = pathlib.Path(sys.argv[1])
s = p.read_text()
old = """    let mut nonce = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce);"""
new = """    let nonce = [0u8; NONCE_LEN]; // SEEDED FAULT: the nonce is a constant
    let _ = OsRng;"""
if old not in s:
    sys.exit("could not find the nonce generation to replace - is hardened.rs unchanged?")
p.write_text(s.replace(old, new))
print("  seeded")
PY

echo
echo "=== do the tests notice? (the hardened properties) ==="
cargo test --quiet --features hardened --test properties 2>&1 | tail -25
status=${PIPESTATUS[0]}

echo
if [ "$status" -ne 0 ]; then
  echo "=== caught. The tests can fail, so they are worth having. Restoring. ==="
else
  echo "=== NOT caught: the tests passed with a seeded fault. They are not tests. ==="
fi
exit "$status"
