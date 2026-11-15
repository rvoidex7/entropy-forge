//! Entropy quality analysis and testing
//!
//! This module provides statistical tests and quality metrics for entropy
//! sources, including NIST SP 800-22 tests.

pub mod bits;
mod metrics;
mod nist;

pub use bits::{bit_at_msb, bytes_to_bits_msb, BitsMsb};
pub use metrics::{QualityMetrics, QualityReport};
pub use nist::NistTests;
