//! Mock entropy source for testing

use super::EntropySource;

/// Mock entropy source for testing
///
/// This source produces predictable output, which is useful for testing
/// and debugging. DO NOT use in production!
///
/// # Examples
///
/// ```
/// use entropy_weaver::entropy::{EntropySource, MockEntropy};
///
/// let mut entropy = MockEntropy::new(42);
/// let mut buffer = [0u8; 4];
/// entropy.fill_bytes(&mut buffer);
/// // buffer now contains predictable bytes based on seed
/// ```
#[derive(Debug, Clone)]
pub struct MockEntropy {
    state: u64,
    initial_state: u64,
}

impl MockEntropy {
    /// Create a new mock entropy source with given seed
    pub fn new(seed: u64) -> Self {
        Self {
            state: seed,
            initial_state: seed,
        }
    }
    
    /// Create with default seed (42)
    pub fn default() -> Self {
        Self::new(42)
    }
    
    // Simple LCG (Linear Congruential Generator)
    // Using glibc's constants
    fn next(&mut self) -> u8 {
        self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
        (self.state >> 24) as u8
    }
}

impl EntropySource for MockEntropy {
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for byte in dest.iter_mut() {
            *byte = self.next();
        }
    }
    
    fn name(&self) -> &str {
        "Mock RNG (for testing only)"
    }
    
    fn reset(&mut self) {
        self.state = self.initial_state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mock_deterministic() {
        let mut entropy1 = MockEntropy::new(42);
        let mut entropy2 = MockEntropy::new(42);
        
        let mut buf1 = [0u8; 32];
        let mut buf2 = [0u8; 32];
        
        entropy1.fill_bytes(&mut buf1);
        entropy2.fill_bytes(&mut buf2);
        
        // Same seed should produce same output
        assert_eq!(buf1, buf2);
    }
    
    #[test]
    fn test_mock_different_seeds() {
        let mut entropy1 = MockEntropy::new(42);
        let mut entropy2 = MockEntropy::new(43);
        
        let mut buf1 = [0u8; 32];
        let mut buf2 = [0u8; 32];
        
        entropy1.fill_bytes(&mut buf1);
        entropy2.fill_bytes(&mut buf2);
        
        // Different seeds should produce different output
        assert_ne!(buf1, buf2);
    }
    
    #[test]
    fn test_mock_reset() {
        let mut entropy = MockEntropy::new(42);
        
        let mut buf1 = [0u8; 32];
        let mut buf2 = [0u8; 32];
        
        entropy.fill_bytes(&mut buf1);
        entropy.reset();
        entropy.fill_bytes(&mut buf2);
        
        // After reset, should produce same output
        assert_eq!(buf1, buf2);
    }
}
