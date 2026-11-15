//! Entropy quality metrics.
//!
//! These metrics go beyond plain Shannon entropy. A deterministic counter
//! (`0, 1, 2, ..., 255`) has perfect Shannon entropy and a perfect mean, yet
//! it has zero cryptographic security. The metrics in this module therefore
//! include conservative min-entropy (NIST SP 800-90B), conditional
//! (Markov-1) min-entropy, lag-1 serial correlation, and a two-sided
//! chi-square anomaly check.

use crate::entropy::EntropySource;
use crate::quality::bits::BitsMsb;
use crate::quality::NistTests;
use std::collections::HashMap;

/// Alias kept for the name used throughout the project documentation.
pub type QualityReport = QualityMetrics;

/// Entropy quality metrics for a byte sequence.
#[derive(Debug, Clone)]
pub struct QualityMetrics {
    /// Shannon entropy in bits per byte (max: 8.0).
    pub shannon_entropy: f64,

    /// NIST SP 800-90B MCV min-entropy using the 99% upper confidence bound.
    pub min_entropy: f64,

    /// Raw (unbounded) MCV min-entropy: `-log2(max p(x))`.
    pub min_entropy_raw: f64,

    /// First-order Markov (conditional) min-entropy in bits per byte.
    ///
    /// This catches deterministic counters and other sources whose byte
    /// distribution is uniform but whose transitions are predictable.
    pub markov_min_entropy: f64,

    /// Lag-1 serial correlation coefficient (should be near 0.0).
    pub serial_correlation: f64,

    /// Frequency of each byte value (0-255).
    pub byte_frequency: HashMap<u8, usize>,

    /// Total bytes analyzed.
    pub total_bytes: usize,

    /// Chi-square statistic for byte uniformity.
    pub chi_square: f64,

    /// Upper-tail p-value of the chi-square statistic (df = 255).
    pub chi_square_p_value: f64,

    /// True when the byte distribution is *too* uniform (`p > 0.9999`).
    ///
    /// Real random sources do not produce a chi-square of exactly zero.
    pub is_suspiciously_uniform: bool,

    /// Number of implemented NIST tests that failed (`p < 0.01`).
    pub nist_failures: usize,

    /// Number of NIST tests that produced a usable result.
    pub nist_tests_run: usize,

    /// Mean byte value (should be ~127.5 for a uniform distribution).
    pub mean: f64,

    /// Longest run of identical bits.
    pub longest_run: usize,
}

impl QualityMetrics {
    /// Minimum number of observed transitions for a previous-byte row before
    /// its conditional probabilities are trusted. See
    /// [`Self::markov_min_entropy`].
    pub const MIN_MARKOV_ROW_OBSERVATIONS: usize = 32;

    /// Calculate Shannon entropy in bits per byte.
    ///
    /// `H(X) = -Σ p(x) log2 p(x)`, maximum 8.0 bits/byte.
    pub fn shannon_entropy(data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }

        let mut frequency: [usize; 256] = [0; 256];
        for &byte in data {
            frequency[byte as usize] += 1;
        }

        let len = data.len() as f64;
        frequency
            .iter()
            .filter(|&&count| count > 0)
            .map(|&count| {
                let p = count as f64 / len;
                -p * p.log2()
            })
            .sum()
    }

    /// NIST SP 800-90B §6.3.1 Most Common Value min-entropy estimate.
    ///
    /// Uses the 99% upper confidence bound `p_u`:
    ///
    /// `p_u = min(1, p_max + 2.576 * sqrt(p_max * (1 - p_max) / (L - 1)))`
    /// `H_inf = -log2(p_u)`
    pub fn min_entropy(data: &[u8]) -> f64 {
        if data.len() < 2 {
            return 0.0;
        }

        let mut frequency: [usize; 256] = [0; 256];
        for &byte in data {
            frequency[byte as usize] += 1;
        }

        let max_count = frequency.iter().copied().max().unwrap_or(0);
        let len = data.len() as f64;
        let p_max = max_count as f64 / len;
        let p_u =
            (p_max + 2.576 * (p_max * (1.0 - p_max) / (data.len() as f64 - 1.0)).sqrt()).min(1.0);

        if p_u > 0.0 {
            -p_u.log2()
        } else {
            0.0
        }
    }

    /// Raw MCV min-entropy without the confidence bound.
    pub fn min_entropy_raw(data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }

        let mut frequency: [usize; 256] = [0; 256];
        for &byte in data {
            frequency[byte as usize] += 1;
        }

        let max_probability =
            frequency.iter().copied().max().unwrap_or(0) as f64 / data.len() as f64;
        if max_probability > 0.0 {
            -max_probability.log2()
        } else {
            0.0
        }
    }

    /// First-order Markov (conditional) min-entropy in bits per byte.
    ///
    /// `H = -log2(max_{a,b} P(s_{i+1} = b | s_i = a))`.
    ///
    /// A deterministic counter yields exactly 0.0 here even though its
    /// Shannon entropy is 8.0.
    ///
    /// The raw plug-in estimator is badly biased upward for small datasets: a
    /// byte value observed once is followed by exactly one value, which would
    /// claim `P = 1.0` and report zero entropy for perfectly good random data.
    /// Rows with fewer than [`Self::MIN_MARKOV_ROW_OBSERVATIONS`] transitions
    /// are therefore ignored. When no row has enough observations the sample
    /// cannot demonstrate predictability and the full 8.0 bits are returned.
    pub fn markov_min_entropy(data: &[u8]) -> f64 {
        if data.len() < 2 {
            return 0.0;
        }

        let mut transitions: HashMap<u16, usize> = HashMap::new();
        let mut row_totals = [0usize; 256];

        for pair in data.windows(2) {
            let from = pair[0] as usize;
            let key = ((pair[0] as u16) << 8) | pair[1] as u16;
            *transitions.entry(key).or_insert(0) += 1;
            row_totals[from] += 1;
        }

        let mut max_probability = 0.0f64;
        let mut qualifying_rows = 0usize;
        for (key, count) in transitions {
            let from = (key >> 8) as usize;
            let total = row_totals[from];
            if total >= Self::MIN_MARKOV_ROW_OBSERVATIONS {
                qualifying_rows += 1;
                max_probability = max_probability.max(count as f64 / total as f64);
            }
        }

        if qualifying_rows == 0 || max_probability <= 0.0 {
            // Not enough transition observations to claim predictability.
            8.0
        } else {
            -max_probability.log2()
        }
    }

    /// Lag-1 serial correlation coefficient (Knuth TAOCP Vol. 2).
    ///
    /// Returns a value in approximately `[-1, 1]`; random data should be close
    /// to 0.0 (`|r| < 3 / sqrt(L)`).
    pub fn serial_correlation(data: &[u8]) -> f64 {
        if data.len() < 2 {
            return 0.0;
        }

        let len = data.len() as f64;
        let mut sum = 0.0;
        let mut sum_sq = 0.0;
        let mut sum_products = 0.0;

        for (index, &byte) in data.iter().enumerate() {
            let value = byte as f64;
            let next = data[(index + 1) % data.len()] as f64;
            sum += value;
            sum_sq += value * value;
            sum_products += value * next;
        }

        let numerator = len * sum_products - sum * sum;
        let denominator = len * sum_sq - sum * sum;
        if denominator == 0.0 {
            0.0
        } else {
            numerator / denominator
        }
    }

    /// Chi-square statistic for byte uniformity (`df = 255`).
    pub fn chi_square(data: &[u8]) -> f64 {
        NistTests::chi_square_statistic(data).max(0.0)
    }

    /// Mean byte value.
    pub fn mean(data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }

        let sum: u64 = data.iter().map(|&byte| byte as u64).sum();
        sum as f64 / data.len() as f64
    }

    /// Longest run of identical bits, using the shared MSB-first reader.
    pub fn longest_run(data: &[u8]) -> usize {
        let mut longest = 0usize;
        let mut current = 0usize;
        let mut previous: Option<u8> = None;

        for bit in BitsMsb::new(data) {
            if previous == Some(bit) {
                current += 1;
            } else {
                current = 1;
                previous = Some(bit);
            }
            longest = longest.max(current);
        }

        longest
    }

    /// Analyze an entropy source by sampling it.
    pub fn analyze<E: ?Sized + EntropySource>(source: &mut E, sample_size: usize) -> Self {
        let mut data = vec![0u8; sample_size];
        source.fill_bytes(&mut data);
        Self::analyze_slice(&data)
    }

    /// Analyze an existing byte slice.
    pub fn analyze_slice(data: &[u8]) -> Self {
        let shannon = Self::shannon_entropy(data);
        let min_entropy = Self::min_entropy(data);
        let min_entropy_raw = Self::min_entropy_raw(data);
        let markov_min_entropy = Self::markov_min_entropy(data);
        let serial_correlation = Self::serial_correlation(data);
        let chi_square = Self::chi_square(data);
        let chi_square_p_value = if data.is_empty() {
            0.0
        } else {
            NistTests::chi_square_test(data)
        };
        let mean_value = Self::mean(data);
        let longest = Self::longest_run(data);

        let mut frequency: HashMap<u8, usize> = HashMap::new();
        for &byte in data {
            *frequency.entry(byte).or_insert(0) += 1;
        }

        // Running the full suite on tiny samples produces meaningless p-values.
        let (nist_failures, nist_tests_run) = if data.len() >= 256 {
            let results = NistTests::run_all_tests(data);
            let run = results.len();
            let failures = results.iter().filter(|(_, p)| *p < 0.01).count();
            (failures, run)
        } else {
            (0, 0)
        };

        Self {
            shannon_entropy: shannon,
            min_entropy,
            min_entropy_raw,
            markov_min_entropy,
            serial_correlation,
            byte_frequency: frequency,
            total_bytes: data.len(),
            chi_square,
            chi_square_p_value,
            is_suspiciously_uniform: chi_square_p_value > 0.9999,
            nist_failures,
            nist_tests_run,
            mean: mean_value,
            longest_run: longest,
        }
    }

    /// Overall quality score from 0 to 100.
    ///
    /// The base score combines Shannon entropy, conservative min-entropy and
    /// mean. A source with predictable structure (deterministic counter,
    /// alternating bit pattern, strong serial correlation, or a majority of
    /// failing NIST tests) can never exceed 30 points.
    pub fn overall_score(&self) -> f64 {
        if self.total_bytes == 0 {
            return 0.0;
        }

        let shannon_score = (self.shannon_entropy / 8.0) * 100.0;
        let min_entropy_score = (self.min_entropy / 8.0) * 100.0;
        let mean_diff = (self.mean - 127.5).abs();
        let mean_score = ((127.5 - mean_diff) / 127.5) * 100.0;

        let base = shannon_score * 0.5 + min_entropy_score * 0.3 + mean_score * 0.2;

        if self.has_artificial_structure() {
            base.min(30.0)
        } else {
            base.clamp(0.0, 100.0)
        }
    }

    /// True when the score was capped because the data looks artificial.
    pub fn is_capped(&self) -> bool {
        self.has_artificial_structure()
    }

    /// Structural red flags that cap the score at 30.
    ///
    /// A 4-sigma serial-correlation bound keeps false positives at about
    /// 0.006%, while a deterministic sequence (`r` close to 1.0) still fails.
    fn has_artificial_structure(&self) -> bool {
        let serial_limit = 4.0 / (self.total_bytes as f64).sqrt();
        let strong_serial_correlation =
            self.total_bytes > 1 && self.serial_correlation.abs() > serial_limit;
        let majority_nist_failure =
            self.nist_tests_run > 0 && self.nist_failures * 2 > self.nist_tests_run;

        self.is_suspiciously_uniform
            || self.markov_min_entropy < 0.5
            || strong_serial_correlation
            || majority_nist_failure
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entropy::SystemEntropy;

    #[test]
    fn shannon_entropy_is_eight_for_full_byte_alphabet() {
        let data: Vec<u8> = (0..=255).collect();
        assert!(QualityMetrics::shannon_entropy(&data) > 7.9);
    }

    #[test]
    fn shannon_entropy_is_zero_for_constant_data() {
        let data = vec![0u8; 256];
        assert_eq!(QualityMetrics::shannon_entropy(&data), 0.0);
    }

    #[test]
    fn min_entropy_is_conservative_for_small_samples() {
        // 4 out of 8 bytes are the same value. The raw estimate is 1.0 bit,
        // the 99% upper bound is naturally lower.
        let data = vec![1, 2, 3, 4, 5, 1, 1, 1];
        let raw = QualityMetrics::min_entropy_raw(&data);
        let conservative = QualityMetrics::min_entropy(&data);
        assert!((raw - 1.0).abs() < 0.01);
        assert!(conservative < raw);
    }

    #[test]
    fn counter_has_perfect_shannon_but_zero_markov_entropy() {
        let counter: Vec<u8> = (0..100_000).map(|i| (i % 256) as u8).collect();
        let metrics = QualityMetrics::analyze_slice(&counter);

        assert!((metrics.shannon_entropy - 8.0).abs() < 0.01);
        assert!(metrics.markov_min_entropy < 0.05);
        assert!(metrics.is_suspiciously_uniform);
        assert!(metrics.overall_score() <= 30.0);
    }

    #[test]
    fn alternating_pattern_scores_low() {
        let data = vec![0xAAu8; 10_000];
        let metrics = QualityMetrics::analyze_slice(&data);
        assert!(metrics.overall_score() <= 30.0);
    }

    #[test]
    fn short_counter_is_still_capped() {
        // Too few transitions per row for the Markov estimator, but the
        // chi-square anomaly and serial correlation must still cap the score.
        let counter: Vec<u8> = (0..10_000).map(|i| (i % 256) as u8).collect();
        let metrics = QualityMetrics::analyze_slice(&counter);
        assert!(metrics.overall_score() <= 30.0);
        assert!(metrics.is_capped());
    }

    #[test]
    fn small_random_sample_is_not_capped() {
        // Regression test: the plug-in Markov estimator used to report zero
        // entropy for a few kilobytes of random data because a byte seen once
        // was followed by exactly one value.
        use crate::crypto::KeyedStreamCipher;

        let zeros = vec![0u8; 2_000];
        let random = KeyedStreamCipher::process_chacha20(&[0x5A; 32], &[0x11; 12], &zeros);
        let metrics = QualityMetrics::analyze_slice(&random);

        assert!(
            metrics.markov_min_entropy > 0.5,
            "random data must not look deterministic, got {}",
            metrics.markov_min_entropy
        );
        assert!(
            !metrics.is_capped(),
            "a random 2 KB sample must not be capped (score {})",
            metrics.overall_score()
        );
        assert!(metrics.overall_score() > 30.0);
    }

    #[test]
    fn mean_is_correct() {
        let data = vec![0, 128, 255];
        assert_eq!(QualityMetrics::mean(&data), (0.0 + 128.0 + 255.0) / 3.0);
    }

    #[test]
    fn system_entropy_scores_well() {
        let mut source = SystemEntropy::new();
        let metrics = QualityMetrics::analyze(&mut source, 100_000);

        assert!(metrics.shannon_entropy > 7.5);
        assert!(metrics.min_entropy > 7.0);
        assert!(metrics.mean > 100.0 && metrics.mean < 155.0);
        assert!(metrics.overall_score() > 80.0);
        assert!(metrics.overall_score() <= 100.0);
    }

    #[test]
    fn longest_run_counts_correctly() {
        assert_eq!(QualityMetrics::longest_run(&[0b0000_0000]), 8);
        assert_eq!(QualityMetrics::longest_run(&[0b0000_0001]), 7);
        assert_eq!(QualityMetrics::longest_run(&[0b1010_1010]), 1);
    }
}
