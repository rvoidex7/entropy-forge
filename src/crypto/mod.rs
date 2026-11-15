//! Cryptographic primitives built on top of the entropy-source system.
//!
//! * [`StreamCipher`] is a deliberately simple one-time-pad style XOR cipher.
//!   It is **malleable** and does **not** have an avalanche effect on the
//!   plaintext; use it only for teaching.
//! * [`KeyedStreamCipher`] is a real ChaCha20 stream cipher with a key and
//!   nonce. Encryption and decryption are exact inverses.
//! * [`AvalancheAnalyzer`] measures the SHA-256 strict avalanche criterion.

mod avalanche;
mod cipher;

pub use avalanche::{AvalancheAnalyzer, AvalancheResult, AvalancheStats};
pub use cipher::{KeyedStreamCipher, StreamCipher};
