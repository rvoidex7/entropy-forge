//! NIST SP 800-22 statistical tests (simplified implementations)

use statrs::distribution::{ChiSquared, ContinuousCDF};

/// NIST SP 800-22 statistical tests
///
/// These tests are used to evaluate the quality of random number generators.
/// Each test returns a p-value, where values ≥ 0.01 indicate the sequence
/// appears random.
///
/// This is a simplified implementation of the full NIST test suite.
pub struct NistTests;

impl NistTests {
    /// Frequency (monobit) test
    ///
    /// Tests if the number of 1s and 0s in the sequence are approximately equal.
    ///
    /// # Arguments
    ///
    /// * `data` - Byte sequence to test
    ///
    /// # Returns
    ///
    /// P-value (0.0 to 1.0). Values ≥ 0.01 indicate randomness.
    pub fn frequency_test(data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        
        let mut sum: i64 = 0;
        
        // Convert to bits and count
        for &byte in data {
            for i in 0..8 {
                if (byte >> i) & 1 == 1 {
                    sum += 1;
                } else {
                    sum -= 1;
                }
            }
        }
        
        let n = (data.len() * 8) as f64;
        let s_obs = (sum as f64).abs() / n.sqrt();
        
        // Calculate p-value using complementary error function
        Self::erfc(s_obs / std::f64::consts::SQRT_2)
    }
    
    /// Runs test
    ///
    /// Tests for proper oscillation between 1s and 0s. Too few or too many
    /// runs indicates non-randomness.
    ///
    /// # Arguments
    ///
    /// * `data` - Byte sequence to test
    ///
    /// # Returns
    ///
    /// P-value (0.0 to 1.0). Values ≥ 0.01 indicate randomness.
    pub fn runs_test(data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        
        // Convert to bits
        let bits: Vec<u8> = data.iter()
            .flat_map(|&byte| (0..8).map(move |i| (byte >> i) & 1))
            .collect();
        
        let n = bits.len() as f64;
        let ones = bits.iter().filter(|&&b| b == 1).count() as f64;
        let pi = ones / n;
        
        // Check pre-test condition
        let threshold = 2.0 / n.sqrt();
        if (pi - 0.5).abs() >= threshold {
            return 0.0; // Test not applicable
        }
        
        // Count runs
        let mut runs = 1;
        for i in 1..bits.len() {
            if bits[i] != bits[i - 1] {
                runs += 1;
            }
        }
        
        let v_obs = runs as f64;
        let numerator = (v_obs - 2.0 * n * pi * (1.0 - pi)).abs();
        let denominator = 2.0 * (2.0 * n).sqrt() * pi * (1.0 - pi);
        
        if denominator == 0.0 {
            return 0.0;
        }
        
        Self::erfc(numerator / (denominator * std::f64::consts::SQRT_2))
    }
    
    /// Longest run of ones test
    ///
    /// Tests the length of the longest run of ones, which shouldn't be
    /// too long in a random sequence.
    pub fn longest_run_test(data: &[u8]) -> f64 {
        if data.len() < 128 {
            return 0.0; // Need at least 128 bytes
        }
        
        // Convert to bits
        let bits: Vec<u8> = data.iter()
            .flat_map(|&byte| (0..8).map(move |i| (byte >> i) & 1))
            .collect();
        
        let n = bits.len();
        
        // Parameters based on sequence length
        let (m, k, v_values, pi_values) = if n < 6272 {
            (8, 3, vec![1, 2, 3, 4], vec![0.2148, 0.3672, 0.2305, 0.1875])
        } else if n < 75000 {
            (128, 5, vec![4, 5, 6, 7, 8, 9], vec![0.1174, 0.2430, 0.2493, 0.1752, 0.1027, 0.1124])
        } else {
            (10000, 6, vec![10, 11, 12, 13, 14, 15, 16], 
             vec![0.0882, 0.2092, 0.2483, 0.1933, 0.1208, 0.0675, 0.0727])
        };
        
        // Divide into blocks
        let num_blocks = n / m;
        let mut v_obs = vec![0usize; v_values.len()];
        
        for block_idx in 0..num_blocks {
            let start = block_idx * m;
            let end = start + m;
            let block = &bits[start..end];
            
            // Find longest run in block
            let mut max_run = 0;
            let mut current_run = 0;
            
            for &bit in block {
                if bit == 1 {
                    current_run += 1;
                    max_run = max_run.max(current_run);
                } else {
                    current_run = 0;
                }
            }
            
            // Categorize the run
            let category = v_values.iter()
                .position(|&v| max_run <= v)
                .unwrap_or(v_values.len() - 1);
            
            v_obs[category] += 1;
        }
        
        // Chi-square statistic
        let mut chi_sq = 0.0;
        for (i, &count) in v_obs.iter().enumerate() {
            let expected = num_blocks as f64 * pi_values[i];
            let diff = count as f64 - expected;
            chi_sq += diff * diff / expected;
        }
        
        // P-value from chi-square distribution
        let df = k as f64;
        if let Ok(dist) = ChiSquared::new(df) {
            1.0 - dist.cdf(chi_sq)
        } else {
            0.0
        }
    }
    
    /// Chi-square test for byte distribution
    ///
    /// Tests whether the byte values are uniformly distributed.
    pub fn chi_square_test(data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        
        let mut freq = vec![0usize; 256];
        for &byte in data {
            freq[byte as usize] += 1;
        }
        
        let n = data.len() as f64;
        let expected = n / 256.0;
        
        let mut chi_square = 0.0;
        for &count in &freq {
            let diff = count as f64 - expected;
            chi_square += diff * diff / expected;
        }
        
        // Chi-square distribution with 255 degrees of freedom
        if let Ok(dist) = ChiSquared::new(255.0) {
            1.0 - dist.cdf(chi_square)
        } else {
            0.0
        }
    }
    
    /// Serial test (two-bit test)
    ///
    /// Tests the frequency of overlapping two-bit patterns.
    pub fn serial_test(data: &[u8]) -> f64 {
        if data.len() < 2 {
            return 0.0;
        }
        
        // Convert to bits
        let bits: Vec<u8> = data.iter()
            .flat_map(|&byte| (0..8).map(move |i| (byte >> i) & 1))
            .collect();
        
        let n = bits.len();
        if n < 2 {
            return 0.0;
        }
        
        // Count 2-bit patterns
        let mut freq = vec![0usize; 4];
        for i in 0..n-1 {
            let pattern = (bits[i] << 1) | bits[i + 1];
            freq[pattern as usize] += 1;
        }
        
        // Chi-square test
        let expected = (n - 1) as f64 / 4.0;
        let mut chi_sq = 0.0;
        
        for &count in &freq {
            let diff = count as f64 - expected;
            chi_sq += diff * diff / expected;
        }
        
        // P-value from chi-square with 3 df
        if let Ok(dist) = ChiSquared::new(3.0) {
            1.0 - dist.cdf(chi_sq)
        } else {
            0.0
        }
    }
    
    /// Run all tests and return results
    ///
    /// Returns a vector of (test_name, p_value) tuples.
    pub fn run_all_tests(data: &[u8]) -> Vec<(&'static str, f64)> {
        vec![
            ("Frequency Test", Self::frequency_test(data)),
            ("Runs Test", Self::runs_test(data)),
            ("Longest Run Test", Self::longest_run_test(data)),
            ("Chi-Square Test", Self::chi_square_test(data)),
            ("Serial Test", Self::serial_test(data)),
        ]
    }
    
    // Helper: Complementary error function
    pub fn erfc(x: f64) -> f64 {
        let z = x.abs();
        let t = 1.0 / (1.0 + 0.5 * z);
        
        let ans = t * (
            -z * z - 1.26551223 +
            t * (1.00002368 +
            t * (0.37409196 +
            t * (0.09678418 +
            t * (-0.18628806 +
            t * (0.27886807 +
            t * (-1.13520398 +
            t * (1.48851587 +
            t * (-0.82215223 +
            t * 0.17087277))))))))
        ).exp();
        
        if x >= 0.0 { ans } else { 2.0 - ans }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entropy::SystemEntropy;
    use crate::entropy::EntropySource;
    
    #[test]
    fn test_frequency_test_uniform() {
        let data = vec![0xAAu8; 1000]; // 10101010 pattern
        let p_value = NistTests::frequency_test(&data);
        // Should pass (close to 50% ones)
        assert!(p_value > 0.01);
    }
    
    #[test]
    fn test_frequency_test_biased() {
        let data = vec![0xFFu8; 1000]; // All ones
        let p_value = NistTests::frequency_test(&data);
        // Should fail (100% ones)
        assert!(p_value < 0.01);
    }
    
    #[test]
    fn test_chi_square() {
        let mut entropy = SystemEntropy::new();
        let mut data = vec![0u8; 10_000];
        entropy.fill_bytes(&mut data);
        
        let p_value = NistTests::chi_square_test(&data);
        // System RNG should pass
        assert!(p_value > 0.01);
    }
    
    #[test]
    fn test_run_all() {
        let mut entropy = SystemEntropy::new();
        let mut data = vec![0u8; 10_000];
        entropy.fill_bytes(&mut data);
        
        let results = NistTests::run_all_tests(&data);
        
        // Should have all tests
        assert_eq!(results.len(), 5);
        
        // Most should pass (allow 1 failure due to statistical variance)
        let passed = results.iter().filter(|(_, p)| *p >= 0.01).count();
        assert!(passed >= 4);
    }
}
