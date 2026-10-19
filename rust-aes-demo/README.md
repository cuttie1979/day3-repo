# rust-aes-demo

> ## ⚠️ Deliberately broken code
>
> The default implementation in this crate (`src/naive.rs`) is **unsafe on purpose**. It compiles,
> it works, its round-trip tests are green, and it must never be copied into anything.
>
> It exists so the difference can be *seen*: the same public API, the same test files, two
> implementations. `src/naive.rs` is what hurried crypto looks like; `src/hardened.rs` is the same
> module with the five flaws gone.

---

## 1. What this is

One crate, two implementations of an AES module — key generation, encryption, decryption, and a
key comparison — selected by a Cargo feature:

```bash
cargo test                                # the naive implementation (default)
cargo test --features hardened            # the hardened implementation
```

The public API and the `tests/` directory are **identical in both states**. Nothing else changes.
That is the entire design: if the tests are the same and the API is the same, then any difference in
outcome comes from what the *types* and the *crates* guarantee.

|  | `src/naive.rs` (default) | `src/hardened.rs` (`--features hardened`) |
|---|---|---|
| cipher | AES-128-CBC, CBC hand-rolled | AES-256-GCM (an AEAD) |
| key size | 16 bytes | 32 bytes |
| IV / nonce | `const IV` — **the same for every message** | 12 random bytes per message |
| message layout | ciphertext blocks only | `nonce ‖ ciphertext ‖ tag` |
| authentication | **none** | the GCM tag (16 bytes) |
| wrong key | accepted, returns garbage | rejected (`AuthFailed`) |
| key comparison | `String == String` | `subtle::ConstantTimeEq` |
| the key type | `Key { hex: String, bytes: [u8; 16] }`, `#[derive(Debug, Clone)]` | `Key(Secret<[u8; 32]>)`, `Debug` redacted, no `Clone` |
| on drop | nothing | zeroized |
| `unsafe` | forbidden crate-wide (`#![forbid(unsafe_code)]`) | the same |

The cipher itself is never written here. AES comes from the vetted `aes` and `aes-gcm` crates
(RustCrypto). What is written by hand is the **glue** — the mode, the nonce, the comparison, the key's
lifetime — and the glue is exactly the part that gets broken.

---

## 2. Why it exists

Because "the tests are green" and "the code is safe" are two different claims, and here that
difference fits on one screen.

The naive implementation passes every test a person writes without thinking:

```bash
cargo test --test roundtrip        # 6 passed
```

and fails every test that describes a *property* rather than a behaviour:

```bash
cargo test --test properties       # 0 passed, 4 failed
```

With `--features hardened`, both are green. Same tests, same API, one flag.

A second reason: **two of the guarantees cannot be tested at all** (section 7). The constant-time
comparison and the wipe on drop produce identical behaviour and identical test results. They have to
come from the type and the crate — never from a test. That is the demonstration.

---

## 3. The demo: eight steps, in order

`./demo.sh` runs all eight. It is the fallback if anything on stage misbehaves. Each step below says
what to run, what appears, and what to say.

### Step 1 — the round-trip tests are green

```bash
cargo test --test roundtrip
```

```
running 6 tests
......
test result: ok. 6 passed; 0 failed
```

Six tests: a short message, exactly one block, a long message, an empty message, the key comparison's
true/false behaviour, and rubbish input. **Say:** *"The agent wrote this. It compiles. It works. The
test is green."*

### Step 2 — the property tests are red

```bash
cargo test --test properties
```

```
the_same_plaintext_twice_looks_different   FAILED
a_tampered_ciphertext_is_rejected          FAILED
the_wrong_key_is_rejected                  FAILED
debug_output_does_not_leak_the_key         FAILED
test result: FAILED. 0 passed; 4 failed
```

**Say:** *"And the code is vulnerable anyway."* Then read one failure aloud, because the message is
the lesson:

```
a modified ciphertext decrypted without complaint, into "transfer 100 to "
```

A tampered message does not produce an error. It produces **plausible-looking garbage** — and the
first sixteen bytes are exactly right, which is the part worth explaining (section 5.2).

### Step 3 and 4 — the same tests, hardened

```bash
cargo test --features hardened --test roundtrip     # 6 passed
cargo test --features hardened --test properties    # 4 passed
```

**Say:** *"The same tests, the same API, one flag. Nothing about the behaviour the round-trip tests
check has changed. What changed is what the types and the crates guarantee."*

### Step 5 and 6 — the key, printed and redacted

```bash
cargo run --example key_debug
cargo run --features hardened --example key_debug
```

```
Key { hex: "7c94d5f445b2889dda2e7c5237bac623", bytes: [124, 148, 213, ...] }
Key(Secret([REDACTED [u8; 32]]))
```

Two lines, side by side. The first is the whole key on screen, from a `#[derive(Debug)]`. The second
is the same key behind a type whose `Debug` refuses to show it. **Say:** *"In the first state you
cannot avoid leaking the key. In the second you cannot leak it by accident — the type will not let
you."*

### Step 7 — a seeded fault, and the tests catch it

```bash
./mutate.sh
```

```
=== the fault: the nonce is no longer random ===
  seeded
=== do the tests notice? (the hardened properties) ===
the_same_plaintext_twice_looks_different --- FAILED
=== caught. The tests can fail, so they are worth having. Restoring. ===
```

The script rewrites `hardened.rs` so the nonce becomes a constant, runs the property tests, and
restores the file whatever happens (a `trap` on exit). The fault it seeds is not cosmetic: **nonce
reuse in GCM is catastrophic** — two messages under the same key and nonce leak the XOR of their
plaintexts, and the authentication key can be recovered. **Say:** *"A check we have never seen fail
is not a check, it is a hope. Now we have seen it."*

### Step 8 — read the diff

```bash
diff -u src/naive.rs src/hardened.rs
```

Three decisions, not more code: the mode, the nonce, the comparison. **Say:** *"The same amount of
code, three decisions made differently. And the agent did not make those decisions — they were in the
contract."*

---

## 4. The five flaws, one by one

### 4.1 The IV is hardcoded

```rust
const IV: [u8; 16] = *b"aicode.page iv!!";          // the same for every message
```

CBC XORs each plaintext block with the previous ciphertext block before encrypting it, and the first
block with the **IV**. The IV exists to make the first block different every time. Fixing it removes
exactly that.

**What leaks.** Under the same key, the same plaintext always produces the same ciphertext. Anyone
watching the wire learns that two messages are *the same message* without decrypting anything. And
two messages that begin with the same bytes produce ciphertexts that begin with the same bytes, which
reveals how much they share.

(Note the precise scope: this is *across messages*. Within one message, CBC does not repeat a
ciphertext block for a repeated plaintext block — that is ECB's failure, a cruder cousin.)

**Caught by:** `the_same_plaintext_twice_looks_different`.

**Hardened:** a fresh 12-byte nonce from the OS RNG for every message, carried in front of the
ciphertext. The nonce is public — it is not a secret — but it must never repeat under one key.

### 4.2 There is no authentication

```rust
pub fn decrypt(k: &Key, ciphertext: &[u8]) -> Result<Vec<u8>, Error> {
    if ciphertext.is_empty() || !ciphertext.len().is_multiple_of(BLOCK) {
        return Err(Error::BadCiphertext);
    }
    Ok(strip_padding(&cbc_decrypt(&k.bytes, &IV, ciphertext)))
}
```

CBC is a *confidentiality-only* mode. It has no tag, no checksum, nothing that ties the ciphertext to
the key. The only structural check is the length, and the padding is stripped by a function that
**believes the last byte and never verifies it**:

```rust
fn strip_padding(data: &[u8]) -> Vec<u8> {
    let n = data.last().copied().unwrap_or(BLOCK as u8).clamp(1, BLOCK as u8) as usize;
    data[..data.len().saturating_sub(n)].to_vec()
}
```

**What an attacker gets.** Two things.

*Malleability.* Flipping bit *j* of ciphertext block *i* corrupts plaintext block *i* into garbage,
and flips **bit *j* of plaintext block *i+1*** — a predictable change in a message the attacker
cannot read. That is how CBC padding oracles and cookie-forging attacks work in the field.

*Silent corruption.* Nothing is reported. The demo tampers with the **last** byte of the ciphertext,
which garbles the last plaintext block; the non-validating strip then cuts an arbitrary number of
bytes and returns a shortened plaintext. The first block is untouched, because in CBC a block's
decryption depends only on its own ciphertext and the previous one. That is why the message reads
`"transfer 100 to "` — sixteen correct bytes, then a silent truncation.

**Caught by:** `a_tampered_ciphertext_is_rejected`.

**Hardened:** AES-GCM is an AEAD — authenticated encryption with associated data. It appends a
16-byte tag computed over the ciphertext and the key. `Aead::decrypt` verifies the tag **before**
releasing any plaintext, so a modified message returns `Err(AuthFailed)` and no bytes at all. There is
no "mostly right" outcome.

### 4.3 The wrong key is not rejected

Same root cause as 4.2: with no tag there is nothing that distinguishes the right key from the wrong
one. A wrong key produces different garbage, the non-validating strip cuts an arbitrary amount, and
the function returns `Ok` with nonsense.

**Caught by:** `the_wrong_key_is_rejected`.

**Hardened:** the tag check rejects it. The same three lines fix both flaws, which is worth saying
aloud: *one design decision removed two failure modes.*

### 4.4 The key is printable

```rust
#[derive(Debug, Clone)]
pub struct Key { hex: String, bytes: [u8; 16] }
```

`Debug` is derived, so it prints every field. A secret that *can* be printed *will* be printed — into
a log line, a panic message, a `{:?}` in a test, a bug report, a screenshot on a call.

**Caught by:** `debug_output_does_not_leak_the_key`, which formats the key and asserts the hex does
not appear in the output.

**Hardened:**

```rust
#[derive(Debug)]
pub struct Key(Secret<[u8; KEY_LEN]>);
```

`secrecy::Secret` implements `Debug` by writing `[REDACTED]` and nothing else. The protection is in
the type, not in the developer's discipline: there is no `{:?}` anywhere that can print this key.
The type is also deliberately **not `Clone`** — copying a secret becomes a decision someone has to
write down, not an accident.

### 4.5 The key is never wiped

In the naive version the key lives in a `String` and an array, and when the `Key` goes out of scope
the bytes stay in memory until that memory is reused. On a machine that swaps, they can reach the
disk. A core dump contains them. Another process reading freed pages may find them.

**Hardened:** `Secret` requires its contents to implement `Zeroize`, and zeroizes them in `Drop`. The
wipe happens because the value is dropped, not because someone remembered to call a function.

**Not testable.** This is the whole point: you cannot assert from a test that the bytes are gone,
because the only way to check would be to read memory the program no longer owns. It is section 7.

---

## 5. The four property tests, one by one

`tests/properties.rs` is identical in both states. Each test names a **property** the module must
have, not a behaviour it must exhibit.

| Test | The property | The naive failure |
|---|---|---|
| `the_same_plaintext_twice_looks_different` | encrypting the same message twice must produce two different ciphertexts | identical bytes, because the IV is fixed |
| `a_tampered_ciphertext_is_rejected` | a modified ciphertext must not decrypt | decrypts into garbage, reported as success |
| `the_wrong_key_is_rejected` | the wrong key must not decrypt | decrypts into garbage, reported as success |
| `debug_output_does_not_leak_the_key` | the key's `Debug` output must not contain the key | it contains the hex and the bytes |

The tamper test is worth reading in full, because it is the shape of every property test here:

```rust
#[test]
fn a_tampered_ciphertext_is_rejected() {
    let key = generate_key();
    let ciphertext = encrypt(&key, b"transfer 100 to Anna");

    let mut tampered = ciphertext.clone();
    let last = tampered.len() - 1;
    tampered[last] ^= 0x01;                       // flip one bit

    match decrypt(&key, &tampered) {
        Err(_) => {}
        Ok(plain) => panic!(
            "a modified ciphertext decrypted without complaint, into {:?}",
            String::from_utf8_lossy(&plain)
        ),
    }
}
```

The `panic!` prints what it *did* return instead. That is deliberate: it is not enough to know a test
failed — you have to know what the code did in its place. That message is where the day's sentence
comes from.

---

## 6. The two guarantees a test cannot reach

### 6.1 Constant-time comparison

```rust
// naive
pub fn key_matches(k: &Key, candidate: &[u8]) -> bool {
    k.hex == to_hex(candidate)
}

// hardened
pub fn key_matches(k: &Key, candidate: &[u8]) -> bool {
    k.0.expose_secret().as_slice().ct_eq(candidate).into()
}
```

Both return the same answers for the same inputs, so **no test can tell them apart**. The difference
is how long they take.

`==` on strings compares byte by byte and **returns at the first difference**. An attacker who can
measure the response time learns how many leading bytes of their guess were right; repeat that a byte
at a time and the secret falls out. This is a real, exploited class of attack, not a theoretical one.

`subtle::ConstantTimeEq` folds every byte comparison into a single accumulator without branching, so
the running time does not depend on *where* the difference is. (The length is still checked first —
a length check cannot be hidden, which is why the comparison is only meaningful for equal-length
secrets.)

**Why it is untestable:** a test can only observe the return value. The leak lives in the time
domain, and timing a function inside a test runner measures the machine, not the algorithm.

### 6.2 The wipe on drop

```rust
#[derive(Debug)]
pub struct Key(Secret<[u8; KEY_LEN]>);
```

`Secret` implements `Drop` to zeroize its contents. **Why it is untestable:** the property is "these
bytes are gone", and by the time you could observe them the value no longer exists. Reading them
would require unsafe code and undefined behaviour — and this crate is
`#![forbid(unsafe_code)]`, deliberately.

There is a second reason it is untestable, and it is a good story: **a naive wipe can be optimised
away.** A plain loop writing zeros over a buffer that is never read again is dead code to the
compiler, and it may simply remove it. The `zeroize` crate writes through `write_volatile` and places
a `compiler_fence` so the writes cannot be reordered or elided.

That is the sentence to say: **in C you have to work this out yourself — and get it right. In Rust
there is a crate for it, and the type system makes you use it.**

---

## 7. What Rust gives you that other languages do not

| Property | In this crate | In C / C++ | In Java, Python, JavaScript, Go |
|---|---|---|---|
| wipe a secret after use | `Secret` zeroizes on drop, in the type | `explicit_bzero` / `SecureZeroMemory`, and you still fight the optimiser | **not reliably possible**: the garbage collector copies objects, strings are immutable, and you do not control the memory |
| the secret cannot be printed | `Debug` is redacted by the type | no such protection | no such protection |
| constant-time comparison | `subtle::ConstantTimeEq` | hand-written, and easy to get wrong | hand-written, and easy to get wrong |
| no memory-safety bug in the glue | `#![forbid(unsafe_code)]` | `unsafe` in crypto glue is the most common source of defects | memory-safe, but the secret is still unwipeable |
| the documentation is a test | a doc example runs under `cargo test` | no equivalent | no equivalent (Python's `doctest` exists but does not run by default) |

The honest framing: **this is not "Rust is better".** It is that a specific set of guarantees is
*cheap* in Rust and *expensive or impossible* elsewhere, and cheap guarantees get used.

---

## 8. The seeded fault

`./mutate.sh` replaces this

```rust
let mut nonce = [0u8; NONCE_LEN];
OsRng.fill_bytes(&mut nonce);
```

with this

```rust
let nonce = [0u8; NONCE_LEN]; // SEEDED FAULT: the nonce is a constant
```

and runs the hardened property tests. One test fails, the file is restored, and the exit code is
non-zero so the script can be used in a pipeline. The point is not that the fault is subtle — it is
that **we have now watched the test fail**, which is the only thing that makes it a test.

---

## 9. Honest limits of this demo

- **No AES is implemented here.** The cipher comes from vetted crates. What is hand-written is the
  glue, which is exactly the part that gets broken.
- **`naive.rs` is not a strawman.** Every flaw in it is a bug that has shipped, in real products, more
  than once. The hardcoded IV in particular is one of the most common real-world crypto mistakes.
- **`key_bytes` hands out a copy.** The hardened state returns a `Vec<u8>` copy of the secret that
  then lives unwiped. Real code would not hand the key out at all; it exists so the tests can
  transport a key.
- **Not constant-time end to end.** Only the comparison is. AES-GCM's internals are constant-time
  inside the crate; the surrounding application is not audited here.
- **Not a security boundary.** This is a teaching artefact, not a library. Do not build anything on
  it.

---

## 10. The files

```text
src/lib.rs               the shared API and the Error type; picks the implementation by feature;
                         #![forbid(unsafe_code)] for the whole crate
src/naive.rs             ⚠️ the broken one: AES-128-CBC, static IV, no tag, printable key, no wipe
src/hardened.rs          the same API: AES-256-GCM, random nonce, Secret, subtle
tests/roundtrip.rs       six tests that pass in both states
tests/properties.rs      four tests that fail in the naive state
examples/key_debug.rs    prints the key's Debug output — run it both ways
mutate.sh                seeds one fault in the hardened implementation, runs the tests, restores
demo.sh                  all eight steps, in order
```

---

## 11. Glossary

| Term | Meaning |
|---|---|
| **IV / nonce** | The per-message value that makes the first block different every time. Public, not secret, but it must never repeat under one key. |
| **CBC** | Cipher Block Chaining: each plaintext block is XORed with the previous ciphertext block before encryption. Confidentiality only. |
| **ECB** | Electronic Codebook: each block encrypted independently. Identical plaintext blocks produce identical ciphertext blocks — the famous "ECB penguin". Never use it. |
| **GCM** | Galois/Counter Mode: an AEAD mode. Fast, and it authenticates. |
| **AEAD** | Authenticated Encryption with Associated Data: confidentiality *and* integrity in one primitive. A modified ciphertext does not decrypt. |
| **tag** | The 16-byte authentication value GCM appends. Verified before any plaintext is released. |
| **PKCS#7 padding** | Filling the last block with N bytes of value N. Correct only if the value is verified on the way out. |
| **constant time** | The running time does not depend on secret data. Required for comparisons of secrets. |
| **zeroize** | Overwriting a secret's memory with zeros so it cannot be recovered afterwards. |
| **mutation testing** | Introducing small faults into the code and checking that the tests fail. A fault that nothing catches marks a missing test. |
| **doc test** | An example inside a documentation comment, executed by `cargo test`. In Rust the documentation *is* a test. |

---

## 12. Questions the room will ask

**"Is this real code?"** No. The naive implementation is deliberately broken and says so at the top
of the file. It exists to be read.

**"Would an agent really write that?"** Yes — that is the point. A hardcoded IV, a missing
authentication tag and a printable key are ordinary mistakes, and the round-trip test passes on all
of them. The agent is not being stupid; it is being *unchecked*.

**"Why not just use a library?"** Use one. The library is `aes-gcm` here. The demonstration is that
the *glue* around the library is still yours to get wrong, and the glue is where the tests are weak.

**"Can't we test the constant-time comparison?"** Not from inside the process. Statistical timing
tests exist but they are noisy and fragile. The practical answer is to use a vetted crate and never
write the comparison yourself.

**"Why is the test green on the naive version?"** Because it only asks "does the round trip work".
The property tests ask "is it safe", and they are the ones that fail.

**"What would catch the fifth flaw?"** A secret scanner. In the day 2 lab a `secretlint` run found a
token in one file and missed it in another because the variable name was neutral — which is its own
lesson: the scanner sees patterns, not intent.

**"What about other languages?"** Python has `hmac.compare_digest` and `cryptography`'s AEAD; Go has
`crypto/subtle` and `cipher.AEAD`; Java has `MessageDigest.isEqual` and `Cipher` with GCM. The modes
and the comparisons are available everywhere. **The wipe is the one that is not** — that is where the
difference is real, and it is why the demo is in Rust.
