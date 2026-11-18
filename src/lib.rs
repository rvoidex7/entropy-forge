//! # Entropy Forge
//!
//! A pluggable entropy framework for cryptographic applications with built-in
//! quality testing and visualization.
//!
//! ## Overview
//!
//! Entropy Forge provides a clean trait-based interface for working with any
//! entropy source (RNG). The framework offers three main capabilities:
//!
//! 1. **Use**: Cryptographic operations (stream cipher, key derivation)
//! 2. **Test**: Quality analysis (NIST tests, entropy metrics)
//! 3. **Benchmark**: Performance measurement
//!
//! ## Quick Start
//!
//! ```rust
//! use entropy_forge::entropy::{EntropySource, SystemEntropy};
//! use entropy_forge::crypto::StreamCipher;
//!
//! let entropy = SystemEntropy::new();
//! let mut cipher = StreamCipher::new(entropy);
//!
//! let plaintext = b"Hello, World!";
//! let ciphertext = cipher.process(plaintext);
//! ```
//!
//! ## Custom Entropy Sources
//!
//! Implement the `EntropySource` trait to plug in your own RNG:
//!
//! ```rust
//! use entropy_forge::entropy::EntropySource;
//!
//! struct MyRNG {
//!     // Your state
//! }
//!
//! impl EntropySource for MyRNG {
//!     fn fill_bytes(&mut self, dest: &mut [u8]) {
//!         // Your implementation
//!     }
//!     
//!     fn name(&self) -> &str {
//!         "My Custom RNG"
//!     }
//! }
//! ```

pub mod bench;
pub mod crypto;
pub mod entropy;
pub mod learn;
pub mod quality;

#[cfg(feature = "gui")]
pub mod viz;

// Re-exports for convenience
pub use crypto::{AvalancheAnalyzer, KeyedStreamCipher, StreamCipher};
pub use entropy::{EntropySource, SourceKind, SystemEntropy};
pub use quality::{NistTests, QualityMetrics, QualityReport};
