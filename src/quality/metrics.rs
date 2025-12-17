//! Entropy quality metrics

use crate::entropy::EntropySource;
use std::collections::HashMap;

/// Entropy quality metrics
///
/// This struct contains various measurements of entropy quality, including
/// Shannon entropy, min-entropy, and byte frequency distribution.
#[derive(Debug, Clone)]
pub struct QualityMetrics {
    /// Shannon entropy in bits per byte (max: 8.0)
    pub shannon_entropy: f64,
    
    /// Min-entropy in bits per byte (conservative estimate)
    pub min_entropy: f64,
    
    /// Frequency of each byte value (0-255)
    pub byte_frequency: HashMap<u8, usize>,
    
    /// Total bytes analyzed
    pub total_bytes: usize,
    
    /// Chi-square statistic (for uniformity test)
    pub chi_square: f64,
    
    /// Mean value (should be ~127.5 for uniform distribution)
    pub mean: f64,
    
    /// Longest run of identical bits
    pub longest_run: usize,
}

impl QualityMetrics {
    /// Calculate Shannon entropy (in bits per byte)
    ///
    /// Shannon entropy measures the average information content.
    /// Higher is better, with 8.0 being perfect for byte-level entropy.
    ///
    /// Formula: H(X) = -Σ p(x) * log₂(p(x))
    pub fn shannon_entropy(data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        
        let mut freq = HashMap::new();
        for &byte in data {
            *freq.entry(byte).or_insert(0) += 1;
        }
        
        let len = data.len() as f64;
        let mut entropy = 0.0;
        
        for count in freq.values() {
            let p = *count as f64 / len;
            if p > 0.0 {
                entropy -= p * p.log2();
            }
        }
        
        entropy
    }
    
    /// Estimate min-entropy (conservative bound)
    ///
    /// Min-entropy is based on the most probable outcome.
    /// It provides a conservative estimate of entropy quality.
    ///
    /// Formula: H_∞(X) = -log₂(max_i p(x_i))
    pub fn min_entropy(data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        
        let mut freq = HashMap::new();
        for &byte in data {
            *freq.entry(byte).or_insert(0) += 1;
        }
        
        let max_freq = freq.values().max().copied().unwrap_or(0);
        let total = data.len() as f64;
        
        if max_freq > 0 {
            -(max_freq as f64 / total).log2()
        } else {
            0.0
        }
    }
    
    /// Calculate chi-square statistic for uniformity
    ///
    /// Tests how well the byte distribution matches a uniform distribution.
    /// Lower values indicate better uniformity.
    pub fn chi_square(data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        
        let mut freq = vec![0usize; 256];
        for &byte in data {
            freq[byte as usize] += 1;
        }
        
        let n = data.len() as f64;
        let expected = n / 256.0;
        
        let mut chi_sq = 0.0;
        for &count in &freq {
            let diff = count as f64 - expected;
            chi_sq += diff * diff / expected;
        }
        
        chi_sq
    }
    
    /// Calculate mean byte value
    pub fn mean(data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        
        let sum: u64 = data.iter().map(|&b| b as u64).sum();
        sum as f64 / data.len() as f64
    }
    
    /// Find longest run of identical bits
    pub fn longest_run(data: &[u8]) -> usize {
        if data.is_empty() {
            return 0;
        }
        
        let mut max_run = 0;
        let mut current_run = 1;
        let mut last_bit = (data[0] >> 7) & 1;
        
        for &byte in data {
            for i in (0..8).rev() {
                let bit = (byte >> i) & 1;
                if bit == last_bit {
                    current_run += 1;
                    max_run = max_run.max(current_run);
                } else {
                    current_run = 1;
                    last_bit = bit;
                }
            }
        }
        
        max_run
    }
    
    /// Analyze entropy source quality
    ///
    /// Generates a full quality report by sampling the entropy source.
    ///
    /// # Arguments
    ///
    /// * `source` - Entropy source to analyze
    /// * `sample_size` - Number of bytes to sample
    ///
    /// # Examples
    ///
    /// ```
    /// use entropy_weaver::entropy::SystemEntropy;
    /// use entropy_weaver::quality::QualityMetrics;
    ///
    /// let mut source = SystemEntropy::new();
    /// let metrics = QualityMetrics::analyze(&mut source, 100_000);
    ///
    /// println!("Shannon Entropy: {:.4} bits/byte", metrics.shannon_entropy);
    /// println!("Min-Entropy: {:.4} bits/byte", metrics.min_entropy);
    /// ```
    pub fn analyze<E: ?Sized + EntropySource>(source: &mut E, sample_size: usize) -> Self {
        let mut data = vec![0u8; sample_size];
        source.fill_bytes(&mut data);
        
        let shannon = Self::shannon_entropy(&data);
        let min_ent = Self::min_entropy(&data);
        let chi_sq = Self::chi_square(&data);
        let mean_val = Self::mean(&data);
        let longest = Self::longest_run(&data);
        
        let mut freq = HashMap::new();
        for &byte in &data {
            *freq.entry(byte).or_insert(0) += 1;
        }
        
        Self {
            shannon_entropy: shannon,
            min_entropy: min_ent,
            byte_frequency: freq,
            total_bytes: sample_size,
            chi_square: chi_sq,
            mean: mean_val,
            longest_run: longest,
        }
    }
    
    /// Get a quality score (0-100)
    ///
    /// Combines multiple metrics into a single score.
    /// 100 is perfect, 0 is worst.
    pub fn overall_score(&self) -> f64 {
        // Shannon entropy score (0-100)
        let shannon_score = (self.shannon_entropy / 8.0) * 100.0;
        
        // Min-entropy score (0-100)
        let min_entropy_score = (self.min_entropy / 8.0) * 100.0;
        
        // Mean score (how close to 127.5)
        let mean_diff = (self.mean - 127.5).abs();
        let mean_score = ((127.5 - mean_diff) / 127.5) * 100.0;
        
        // Weighted average
        shannon_score * 0.5 + min_entropy_score * 0.3 + mean_score * 0.2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entropy::{SystemEntropy, MockEntropy};
    
    #[test]
    fn test_shannon_entropy_perfect() {
        // All different bytes -> high entropy
        let data: Vec<u8> = (0..=255).collect();
        let entropy = QualityMetrics::shannon_entropy(&data);
        assert!(entropy > 7.9); // Should be close to 8.0
    }
    
    #[test]
    fn test_shannon_entropy_low() {
        // All same bytes -> zero entropy
        let data = vec![0u8; 256];
        let entropy = QualityMetrics::shannon_entropy(&data);
        assert_eq!(entropy, 0.0);
    }
    
    #[test]
    fn test_min_entropy() {
        let data = vec![1, 2, 3, 4, 5, 1, 1, 1];
        let min_ent = QualityMetrics::min_entropy(&data);
        // 4 out of 8 are '1', so max prob is 0.5
        // min_entropy = -log2(0.5) = 1.0
        assert!((min_ent - 1.0).abs() < 0.01);
    }
    
    #[test]
    fn test_mean() {
        let data = vec![0, 128, 255];
        let mean = QualityMetrics::mean(&data);
        assert_eq!(mean, (0.0 + 128.0 + 255.0) / 3.0);
    }
    
    #[test]
    fn test_analyze_system() {
        let mut source = SystemEntropy::new();
        let metrics = QualityMetrics::analyze(&mut source, 10_000);
        
        // System RNG should have good entropy
        assert!(metrics.shannon_entropy > 7.5);
        assert!(metrics.min_entropy > 7.0);
        assert!(metrics.mean > 100.0 && metrics.mean < 155.0);
    }
    
    #[test]
    fn test_overall_score() {
        let mut source = SystemEntropy::new();
        let metrics = QualityMetrics::analyze(&mut source, 10_000);
        
        let score = metrics.overall_score();
        assert!(score > 80.0); // System RNG should score well
        assert!(score <= 100.0);
    }
}
