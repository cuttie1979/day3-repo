//! The tests that **fail against the naive implementation**.
//!
//! These are the security properties, not the functionality. Every one of them is a test that
//! *can* fail — which is the only kind worth having.
//!
//! ```text
//! cargo test --test properties                        # RED  (naive)
//! cargo test --features hardened --test properties     # green
//! ```

use rust_aes_demo::{decrypt, encrypt, generate_key, key_bytes};

/// The IV is hardcoded, so the same plaintext under the same key always produces the same
/// ciphertext. Anyone watching the wire learns that two messages are the same message.
#[test]
fn the_same_plaintext_twice_looks_different() {
    let key = generate_key();
    let message = b"transfer 100 to Anna";
    let first = encrypt(&key, message);
    let second = encrypt(&key, message);
    assert_ne!(
        first, second,
        "the same plaintext produced the same ciphertext: the IV is reused"
    );
}

/// Nothing authenticates the ciphertext, so an edited message decrypts into garbage and is
/// reported as a success.
#[test]
fn a_tampered_ciphertext_is_rejected() {
    let key = generate_key();
    let ciphertext = encrypt(&key, b"transfer 100 to Anna");

    let mut tampered = ciphertext.clone();
    let last = tampered.len() - 1;
    tampered[last] ^= 0x01;

    match decrypt(&key, &tampered) {
        Err(_) => {}
        Ok(plain) => panic!(
            "a modified ciphertext decrypted without complaint, into {:?}",
            String::from_utf8_lossy(&plain)
        ),
    }
}

/// Same reason: without a tag there is nothing to tell the wrong key from the right one.
#[test]
fn the_wrong_key_is_rejected() {
    let key = generate_key();
    let other = generate_key();
    let ciphertext = encrypt(&key, b"transfer 100 to Anna");

    match decrypt(&other, &ciphertext) {
        Err(_) => {}
        Ok(plain) => panic!(
            "the wrong key decrypted the message into {:?}",
            String::from_utf8_lossy(&plain)
        ),
    }
}

/// The key is a `String` with a derived `Debug`, so it ends up in logs, in panic messages and in
/// bug reports. A secret that can be printed will be printed.
#[test]
fn debug_output_does_not_leak_the_key() {
    let key = generate_key();
    let hex: String = key_bytes(&key).iter().map(|b| format!("{b:02x}")).collect();
    let printed = format!("{key:?}");
    assert!(
        !printed.contains(&hex),
        "the key's own Debug output contains the key: {printed}"
    );
}
