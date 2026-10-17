# rust-aes-demo

> ## ⚠️ Deliberately broken code
>
> The default implementation in this crate is **unsafe on purpose**. It compiles, it works, its
> round-trip tests are green, and it must never be copied into anything.
>
> It exists so the difference can be *seen*: the same public API, the same test files, two
> implementations. `src/naive.rs` is what hurried crypto looks like; `src/hardened.rs` is the same
> module with the five flaws gone.

## Why this crate exists

Because "the tests are green" and "the code is safe" are different claims, and here that difference
fits on one screen. Same tests, one flag:

| | `cargo test` (naive) | `cargo test --features hardened` |
|---|---|---|
| `tests/roundtrip.rs` — it works | **green** | **green** |
| `tests/properties.rs` — it is safe | **RED** | **green** |

## The two states

| | `naive.rs` | `hardened.rs` |
|---|---|---|
| mode | AES-128-CBC, hand-rolled | AES-256-GCM (AEAD) |
| IV / nonce | **hardcoded**, the same for every message | random 96-bit, one per message |
| authentication | none | the GCM tag |
| key comparison | `String == String` | `subtle::ConstantTimeEq` |
| the key itself | a `String`, `Debug` derived | `Secret<[u8; 32]>`, `Debug` redacted, zeroized on drop |
| layout | blocks | `nonce ‖ ciphertext ‖ tag` |

The public API is identical in both, and `tests/` is identical in both. Nothing else changes.

## The demo, in order

```bash
cargo test --test roundtrip                        # 1  green
cargo test --test properties                       # 2  RED — four failures
cargo test --features hardened --test roundtrip     # 3  green
cargo test --features hardened --test properties    # 4  green
cargo run --example key_debug                       # 5  the key, printed
cargo run --features hardened --example key_debug    # 6  redacted
./mutate.sh                                         # 7  seed a fault, the tests catch it
diff -u src/naive.rs src/hardened.rs                # 8  read the diff
```

`./demo.sh` runs all eight in this order. It is the fallback if anything on the day misbehaves.

### What each step says

**1–2.** *"The agent wrote this. It compiles, it works, the tests are green."* Then the properties
go red: the same plaintext twice produces the same ciphertext, a tampered message decrypts into
plausible-looking garbage, the wrong key is accepted, and the key prints itself.

**3–4.** The same tests, the same API, one flag. Nothing about the *behaviour* the round-trip tests
check has changed; what changed is what the types and the crates guarantee.

**5–6.** Two lines, side by side:

```text
Key { hex: "a6d934e5b6e8d27e301e2cb29178d369", bytes: [166, 217, 52, 229, …] }
Key(Secret([REDACTED [u8; 32]]))
```

**7.** A fault is seeded into the hardened implementation — the nonce stops being random — and the
property tests catch it. *A check we have never seen fail is not a check, it is a hope.*

**8.** The diff is the lesson: the mode, the nonce, the comparison, the key type.

## The five flaws in `naive.rs`

1. **The IV is hardcoded** — the same plaintext under the same key always produces the same
   ciphertext. Anyone on the wire learns that two messages are the same message.
2. **No authentication** — a modified ciphertext decrypts into garbage and reports success.
3. **The wrong key is not rejected**, for the same reason.
4. **The key is a printable `String`** — it ends up in logs, in panic messages, in bug reports.
5. **The key is never wiped** from memory when it goes out of scope.

Flaws 1–3 are *behavioural*: a test can catch them. Flaws 4–5 are not.

## The point: two guarantees a test cannot reach

- **Constant-time comparison.** `key_matches` behaves identically in both states — same answers,
  same test results. The difference is *how long it takes*, and that is not a test's business.
- **The wipe on drop.** The whole point is that the bytes cannot be read afterwards.

> Some guarantees can only come from the type and the crate, never from a test.

That is the answer to *"guaranteed quality?"*. For behaviour, write the test. For structure, choose
the type. `Secret` and `subtle` are that choice, and they are one line each.

## Honest limits of this demo

- **No AES is implemented here.** The cipher comes from vetted crates (`aes`, `aes-gcm`). What is
  written by hand is the glue — the mode, the nonce, the comparison, the key's lifetime — which is
  exactly the part that gets broken.
- **Not constant-time end to end.** `key_bytes` returns a `Vec<u8>` copy of the secret that then
  lives unwiped. Real code would not hand the key out at all.
- **`naive.rs` is not a strawman.** Every flaw in it is a bug that has shipped, in real products,
  more than once.

## Layout

```text
src/lib.rs          the shared API and the error type; picks the implementation by feature
src/naive.rs        ⚠️ the broken one
src/hardened.rs     the same API, hardened
tests/roundtrip.rs  the tests that pass in both states
tests/properties.rs the tests that fail in the naive state
examples/key_debug.rs  prints the key's Debug output
mutate.sh           seeds one fault in the hardened implementation and restores it
demo.sh             all eight steps, in order
```
