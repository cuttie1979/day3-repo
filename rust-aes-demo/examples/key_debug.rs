//! Prints the key's `Debug` output. Run it both ways:
//!
//! ```text
//! cargo run --quiet --example key_debug                      # the key, in full, in your terminal
//! cargo run --quiet --features hardened --example key_debug   # redacted
//! ```

fn main() {
    let key = rust_aes_demo::generate_key();
    println!("{key:?}");
}
