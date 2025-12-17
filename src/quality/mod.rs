//! Entropy quality analysis and testing
//!
//! This module provides statistical tests and quality metrics for entropy
//! sources, including NIST SP 800-22 tests.

mod metrics;
mod nist;

pub use metrics::QualityMetrics;
pub use nist::NistTests;
