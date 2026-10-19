//! # HARDENED — the same API, the same tests, and the five flaws gone
//!
//! Compare it with `naive.rs` line by line; the public functions have the same signatures and the
//! test files are unchanged. What differs is what the types and the crates guarantee:
//!
//! | flaw in `naive.rs` | here |
//! |---|---|
//! | hardcoded IV | a random 96-bit nonce per message, carried in front of the ciphertext |
//! | no authentication | AES-256-GCM: a modified ciphertext does not decrypt, it returns `AuthFailed` |
//! | the wrong key not rejected | the same tag check rejects it |
//! | the key is a printable `String` | `Secret`, whose `Debug` is redacted |
//! | the key is never wiped | `Secret` zeroizes on drop, and it cannot be copied by accident |
//!
//! Two properties here **cannot be tested**: the constant-time comparison and the wipe on drop.
//! That is the point of the exercise — some guarantees can only come from the type and the crate,
//! never from a test.

use crate::Error;
use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use rand_core::{OsRng, RngCore};
use secrecy::{ExposeSecret, Secret};
use subtle::ConstantTimeEq;

const KEY_LEN: usize = 32;
const NONCE_LEN: usize = 12;
const TAG_LEN: usize = 16;

/// The key cannot be printed (`Debug` is redacted) and is zeroized when it is dropped.
/// It is deliberately not `Clone`: copying a secret is a decision, not an accident.
#[derive(Debug)]
pub struct Key(Secret<[u8; KEY_LEN]>);

pub fn generate_key() -> Key {
    let mut bytes = [0u8; KEY_LEN];
    OsRng.fill_bytes(&mut bytes);
    Key(Secret::new(bytes))
}

pub fn key_bytes(k: &Key) -> Vec<u8> {
    k.0.expose_secret().to_vec()
}

/// Constant time: the comparison does not stop early, so how long it takes says nothing about how
/// many bytes were right. (Untestable — see the module docs.)
pub fn key_matches(k: &Key, candidate: &[u8]) -> bool {
    k.0.expose_secret().as_slice().ct_eq(candidate).into()
}

/// Layout: `nonce || ciphertext || tag`. The nonce is public and travels with the message.
pub fn encrypt(k: &Key, plaintext: &[u8]) -> Vec<u8> {
    let cipher = Aes256Gcm::new_from_slice(k.0.expose_secret().as_slice())
        .expect("a 32-byte key is always valid for AES-256-GCM");
    let mut nonce = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce);
    let sealed = cipher
        .encrypt(Nonce::from_slice(&nonce), plaintext)
        .expect("encryption does not fail");
    let mut out = Vec::with_capacity(NONCE_LEN + sealed.len());
    out.extend_from_slice(&nonce);
    out.extend_from_slice(&sealed);
    out
}

pub fn decrypt(k: &Key, ciphertext: &[u8]) -> Result<Vec<u8>, Error> {
    if ciphertext.len() < NONCE_LEN + TAG_LEN {
        return Err(Error::BadCiphertext);
    }
    let (nonce, sealed) = ciphertext.split_at(NONCE_LEN);
    let cipher = Aes256Gcm::new_from_slice(k.0.expose_secret().as_slice())
        .map_err(|_| Error::BadCiphertext)?;
    cipher
        .decrypt(Nonce::from_slice(nonce), sealed)
        .map_err(|_| Error::AuthFailed)
}
