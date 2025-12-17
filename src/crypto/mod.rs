//! Cryptographic operations using entropy sources
//!
//! This module provides cryptographic primitives that use the pluggable
//! entropy source system.

mod cipher;

pub use cipher::StreamCipher;
