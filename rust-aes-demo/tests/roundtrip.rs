//! The tests that pass in **both** states.
//!
//! This is the file the naive implementation is green against, and that is exactly the lesson:
//! a green suite here says the module works, not that it is safe.

use rust_aes_demo::{decrypt, encrypt, generate_key, key_bytes, key_matches};

#[test]
fn round_trip_short() {
    let key = generate_key();
    let message = b"hello world";
    let ciphertext = encrypt(&key, message);
    assert_eq!(decrypt(&key, &ciphertext).unwrap(), message);
}

#[test]
fn round_trip_exactly_one_block() {
    let key = generate_key();
    let message = b"0123456789abcdef";
    let ciphertext = encrypt(&key, message);
    assert_eq!(decrypt(&key, &ciphertext).unwrap(), message);
}

#[test]
fn round_trip_long() {
    let key = generate_key();
    let message: Vec<u8> = (0..1000u32).map(|i| (i % 251) as u8).collect();
    let ciphertext = encrypt(&key, &message);
    assert_eq!(decrypt(&key, &ciphertext).unwrap(), message);
}

#[test]
fn round_trip_empty() {
    let key = generate_key();
    let ciphertext = encrypt(&key, b"");
    assert_eq!(decrypt(&key, &ciphertext).unwrap(), b"");
}

#[test]
fn a_matching_key_matches_and_another_does_not() {
    let key = generate_key();
    let bytes = key_bytes(&key);
    assert!(key_matches(&key, &bytes));

    let mut other = bytes.clone();
    other[0] ^= 0x01;
    assert!(!key_matches(&key, &other));

    assert!(!key_matches(&key, b"too short"));
}

#[test]
fn rubbish_is_not_a_ciphertext() {
    let key = generate_key();
    assert!(decrypt(&key, b"").is_err());
    assert!(decrypt(&key, b"short").is_err());
}
