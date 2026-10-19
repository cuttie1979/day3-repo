// The whole crate is declared free of `unsafe`: neither state can reach for a raw pointer, and the
// compiler enforces it. It is one of the things Rust gives a crypto module.
#![forbid(unsafe_code)]

//! # rust-aes-demo — teaching material
//!
//! **Deliberately broken code lives in this crate.** The default implementation (`naive.rs`) is
//! written the way a coding agent writes crypto when nobody checks it: it compiles, it round-trips,
//! its tests are green — and it is unsafe. It exists to be read, not used.
//!
//! **Never copy the naive implementation into anything.** It is here so that the difference can be
//! seen, measured and understood.

//!
//! ## The two states
//!
//! | | default (`naive.rs`) | `--features hardened` (`hardened.rs`) |
//! |---|---|---|
//! | mode | AES-128-CBC | AES-256-GCM (AEAD) |
//! | IV / nonce | **hardcoded**, the same for every message | random, one per message |
//! | authentication | none | the GCM tag |
//! | key comparison | `String == String` | `subtle::ConstantTimeEq` |
//! | the key itself | a `String` with a derived `Debug` | `SecretBox`, redacted `Debug` |
//! | on drop | nothing | `ZeroizeOnDrop` |
//!
//! The public API is identical in both states, and the test files are identical: `cargo test`
//! and `cargo test --features hardened` run the same code against the same tests.
//!
//! ## The demo
//!
//! ```text
//! cargo test --test roundtrip                        # green in BOTH states
//! cargo test --test properties                       # RED in the naive state
//! cargo test --features hardened --test properties    # green
//! ./mutate.sh                                        # the hardened tests catch a seeded fault
//! ```
//!
//! See `README.md` for the full run order and what to say at each step.

#[cfg(not(feature = "hardened"))]
#[path = "naive.rs"]
mod imp;

#[cfg(feature = "hardened")]
#[path = "hardened.rs"]
mod imp;

pub use imp::*;

/// What can go wrong. The naive state never returns [`Error::AuthFailed`] — it has nothing to
/// authenticate with, so a tampered ciphertext simply decrypts into garbage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// The input is not a well-formed ciphertext for this scheme.
    BadCiphertext,
    /// The ciphertext was altered, or the wrong key was used (hardened state only).
    AuthFailed,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BadCiphertext => write!(f, "not a well-formed ciphertext"),
            Error::AuthFailed => write!(f, "the ciphertext did not authenticate"),
        }
    }
}

impl std::error::Error for Error {}
