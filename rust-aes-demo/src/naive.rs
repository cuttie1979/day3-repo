//! # NAIVE — deliberately broken
//!
//! **Teaching material. Never use this.**
//!
//! This is what the code looks like when it is written quickly and only the round-trip is tested.
//! It compiles, it works, and it is unsafe in five separate ways:
//!
//! 1. **The IV is hardcoded.** Every message is encrypted with the same IV, so the same plaintext
//!    under the same key always produces the same ciphertext. That is a real information leak, and
//!    it is the reason the first property test fails.
//! 2. **There is no authentication.** Nothing detects a modified ciphertext: decryption returns
//!    garbage and says it succeeded.
//! 3. **The wrong key is not rejected**, for the same reason.
//! 4. **The key is a `String`** with a derived `Debug`, so it appears in logs and in panic output.
//! 5. **The key is never wiped** from memory when it goes out of scope.
//!
//! And the point of the exercise: `tests/roundtrip.rs` is **green** against this file.

use crate::Error;
use aes::cipher::{BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes128;
use rand_core::{OsRng, RngCore};

/// Flaw 1: the IV is a constant. The same for every message, for the lifetime of the program.
const IV: [u8; 16] = *b"aicode.page iv!!";

const BLOCK: usize = 16;

/// Flaw 4: the key is a `String`, and `Debug` is derived, so it prints.
#[derive(Debug, Clone)]
pub struct Key {
    hex: String,
    bytes: [u8; 16],
}

pub fn generate_key() -> Key {
    let mut bytes = [0u8; 16];
    OsRng.fill_bytes(&mut bytes);
    Key {
        hex: to_hex(&bytes),
        bytes,
    }
}

pub fn key_bytes(k: &Key) -> Vec<u8> {
    k.bytes.to_vec()
}

/// Flaw: `String == String` is not constant time — it stops at the first differing byte, so how
/// long it takes says how many bytes were right.
pub fn key_matches(k: &Key, candidate: &[u8]) -> bool {
    k.hex == to_hex(candidate)
}

pub fn encrypt(k: &Key, plaintext: &[u8]) -> Vec<u8> {
    cbc_encrypt(&k.bytes, &IV, &pkcs7_pad(plaintext))
}

/// Flaws 2 and 3: there is nothing to check, so anything of the right shape decrypts "successfully".
pub fn decrypt(k: &Key, ciphertext: &[u8]) -> Result<Vec<u8>, Error> {
    if ciphertext.is_empty() || !ciphertext.len().is_multiple_of(BLOCK) {
        return Err(Error::BadCiphertext);
    }
    Ok(strip_padding(&cbc_decrypt(&k.bytes, &IV, ciphertext)))
}

/* ------------------------------------------------------------------ glue */

fn cbc_encrypt(key: &[u8; 16], iv: &[u8; 16], data: &[u8]) -> Vec<u8> {
    let cipher = Aes128::new(key.into());
    let mut prev = *iv;
    let mut out = Vec::with_capacity(data.len());
    for chunk in data.chunks(BLOCK) {
        let mut block = [0u8; BLOCK];
        for i in 0..BLOCK {
            block[i] = chunk[i] ^ prev[i];
        }
        cipher.encrypt_block((&mut block).into());
        out.extend_from_slice(&block);
        prev = block;
    }
    out
}

fn cbc_decrypt(key: &[u8; 16], iv: &[u8; 16], data: &[u8]) -> Vec<u8> {
    let cipher = Aes128::new(key.into());
    let mut prev = *iv;
    let mut out = Vec::with_capacity(data.len());
    for chunk in data.chunks(BLOCK) {
        let mut block = [0u8; BLOCK];
        block.copy_from_slice(chunk);
        let ciphertext_block = block;
        cipher.decrypt_block((&mut block).into());
        for i in 0..BLOCK {
            block[i] ^= prev[i];
        }
        out.extend_from_slice(&block);
        prev = ciphertext_block;
    }
    out
}

fn pkcs7_pad(data: &[u8]) -> Vec<u8> {
    let pad = BLOCK - (data.len() % BLOCK);
    let mut out = data.to_vec();
    out.resize(out.len() + pad, pad as u8);
    out
}

/// Strips padding the way a hurried implementation does: it believes the last byte, and it does
/// not verify it. A tampered ciphertext therefore produces garbage instead of an error.
fn strip_padding(data: &[u8]) -> Vec<u8> {
    let n = data
        .last()
        .copied()
        .unwrap_or(BLOCK as u8)
        .clamp(1, BLOCK as u8) as usize;
    let cut = data.len().saturating_sub(n);
    data[..cut].to_vec()
}

fn to_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{b:02x}"));
    }
    s
}
