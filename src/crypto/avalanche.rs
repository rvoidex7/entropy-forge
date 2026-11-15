//! SHA-256 strict avalanche criterion (SAC) analysis.
//!
//! The avalanche effect is a property of hash functions and block ciphers, not
//! of a plaintext-XOR stream cipher. Flipping one input bit of SHA-256 should
//! flip approximately half of the 256 output bits.

use sha2::{Digest, Sha256};

/// Result of flipping a single input bit.
#[derive(Debug, Clone, PartialEq)]
pub struct AvalancheResult {
    /// Original SHA-256 digest.
    pub original: [u8; 32],
    /// Digest after flipping one input bit.
    pub modified: [u8; 32],
    /// Index of the flipped input bit (MSB-first).
    pub flipped_bit: usize,
    /// Number of output bits that changed.
    pub changed_bits: usize,
    /// Total output bits (256 for SHA-256).
    pub total_bits: usize,
    /// Percentage of output bits that changed.
    pub flip_percentage: f64,
}

/// Aggregate statistics over many single-bit flips.
#[derive(Debug, Clone, PartialEq)]
pub struct AvalancheStats {
    /// Number of experiments performed.
    pub samples: usize,
    /// Mean percentage of changed output bits.
    pub mean_flip_percentage: f64,
    /// Smallest percentage observed.
    pub min_flip_percentage: f64,
    /// Largest percentage observed.
    pub max_flip_percentage: f64,
    /// Mean number of changed output bits.
    pub mean_changed_bits: f64,
}

impl Default for AvalancheStats {
    fn default() -> Self {
        Self {
            samples: 0,
            mean_flip_percentage: 0.0,
            min_flip_percentage: 0.0,
            max_flip_percentage: 0.0,
            mean_changed_bits: 0.0,
        }
    }
}

/// SHA-256 strict avalanche criterion analyzer.
pub struct AvalancheAnalyzer;

impl AvalancheAnalyzer {
    /// Compute the SHA-256 digest of `data`.
    pub fn sha256(data: &[u8]) -> [u8; 32] {
        let digest = Sha256::digest(data);
        let mut output = [0u8; 32];
        output.copy_from_slice(&digest);
        output
    }

    /// Flip one input bit (MSB-first) and compare the two digests.
    ///
    /// Returns `None` when `data` is empty or `bit_index` is out of range.
    pub fn compare_single_bit(data: &[u8], bit_index: usize) -> Option<AvalancheResult> {
        if data.is_empty() || bit_index >= data.len() * 8 {
            return None;
        }

        let original = Self::sha256(data);

        let mut modified_data = data.to_vec();
        modified_data[bit_index / 8] ^= 1 << (7 - (bit_index % 8));
        let modified = Self::sha256(&modified_data);

        let changed_bits = original
            .iter()
            .zip(&modified)
            .map(|(a, b)| (a ^ b).count_ones() as usize)
            .sum();

        Some(AvalancheResult {
            original,
            modified,
            flipped_bit: bit_index,
            changed_bits,
            total_bits: 256,
            flip_percentage: (changed_bits as f64 / 256.0) * 100.0,
        })
    }

    /// Run up to 200 single-bit flips and summarize the avalanche behavior.
    ///
    /// For SHA-256 the ideal mean is 50.0% (128 of 256 bits), with a standard
    /// deviation of `sqrt(256 * 0.25) = 8` bits per experiment.
    pub fn analyze_sha256(data: &[u8]) -> AvalancheStats {
        let total_bits = data.len() * 8;
        if total_bits == 0 {
            return AvalancheStats::default();
        }

        let samples = total_bits.min(200);
        let step = (total_bits / samples).max(1);

        let mut sum_percentage = 0.0;
        let mut sum_changed = 0.0;
        let mut min_percentage = f64::INFINITY;
        let mut max_percentage = f64::NEG_INFINITY;
        let mut completed = 0usize;

        for sample in 0..samples {
            let bit_index = (sample * step) % total_bits;
            if let Some(result) = Self::compare_single_bit(data, bit_index) {
                sum_percentage += result.flip_percentage;
                sum_changed += result.changed_bits as f64;
                min_percentage = min_percentage.min(result.flip_percentage);
                max_percentage = max_percentage.max(result.flip_percentage);
                completed += 1;
            }
        }

        if completed == 0 {
            return AvalancheStats::default();
        }

        AvalancheStats {
            samples: completed,
            mean_flip_percentage: sum_percentage / completed as f64,
            min_flip_percentage: min_percentage,
            max_flip_percentage: max_percentage,
            mean_changed_bits: sum_changed / completed as f64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_avalanche_mean_is_near_fifty_percent() {
        let stats = AvalancheAnalyzer::analyze_sha256(b"Entropy Forge Avalanche Benchmark");
        assert_eq!(stats.samples, 200);
        assert!(
            (stats.mean_flip_percentage - 50.0).abs() < 3.0,
            "mean = {:.2}%",
            stats.mean_flip_percentage
        );
        assert!(stats.min_flip_percentage <= stats.mean_flip_percentage);
        assert!(stats.max_flip_percentage >= stats.mean_flip_percentage);
    }

    #[test]
    fn single_bit_flip_changes_roughly_half_the_digest() {
        let result = AvalancheAnalyzer::compare_single_bit(b"Entropy Forge", 0).unwrap();
        assert_eq!(result.total_bits, 256);
        assert!(result.changed_bits > 80 && result.changed_bits < 176);
        assert_eq!(result.original, AvalancheAnalyzer::sha256(b"Entropy Forge"));
        assert_ne!(result.original, result.modified);
    }

    #[test]
    fn empty_input_is_handled() {
        assert_eq!(
            AvalancheAnalyzer::analyze_sha256(b""),
            AvalancheStats::default()
        );
        assert!(AvalancheAnalyzer::compare_single_bit(b"", 0).is_none());
    }
}
