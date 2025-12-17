//! Entropy source trait and implementations
//!
//! This module defines the core `EntropySource` trait that allows any RNG
//! to be used throughout the Entropy Forge framework.

mod system;
mod mock;

pub use system::SystemEntropy;
pub use mock::MockEntropy;

/// Core trait for entropy sources
///
/// Any random number generator can implement this trait to be used with
/// Entropy Forge. The framework will use your RNG for cryptographic
/// operations, quality testing, and benchmarking.
///
/// # Examples
///
/// ```
/// use entropy_forge::entropy::EntropySource;
///
/// struct MyRNG {
///     state: u64,
/// }
///
/// impl EntropySource for MyRNG {
///     fn fill_bytes(&mut self, dest: &mut [u8]) {
///         for byte in dest.iter_mut() {
///             // Simple LCG for example (DO NOT use in production!)
///             self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
///             *byte = (self.state >> 24) as u8;
///         }
///     }
///     
///     fn name(&self) -> &str {
///         "My Custom RNG"
///     }
/// }
/// ```
pub trait EntropySource {
    /// Fill the destination buffer with random bytes
    ///
    /// This is the main method that must be implemented. All other methods
    /// have default implementations that use this.
    ///
    /// # Arguments
    ///
    /// * `dest` - Buffer to fill with random bytes
    fn fill_bytes(&mut self, dest: &mut [u8]);
    
    /// Get a single random byte
    ///
    /// Default implementation uses `fill_bytes`. Override if you have
    /// a more efficient method.
    fn next_byte(&mut self) -> u8 {
        let mut buf = [0u8; 1];
        self.fill_bytes(&mut buf);
        buf[0]
    }
    
    /// Get a random u32
    ///
    /// Default implementation uses `fill_bytes`. Override if you have
    /// a more efficient method.
    fn next_u32(&mut self) -> u32 {
        let mut buf = [0u8; 4];
        self.fill_bytes(&mut buf);
        u32::from_le_bytes(buf)
    }
    
    /// Get a random u64
    ///
    /// Default implementation uses `fill_bytes`. Override if you have
    /// a more efficient method.
    fn next_u64(&mut self) -> u64 {
        let mut buf = [0u8; 8];
        self.fill_bytes(&mut buf);
        u64::from_le_bytes(buf)
    }
    
    /// Get the source name for display purposes
    ///
    /// This is used in the GUI and logging to identify which entropy
    /// source is currently active.
    fn name(&self) -> &str {
        "Unknown Source"
    }
    
    /// Optional: Reset the source to initial state
    ///
    /// Some sources may support resetting to a known state for testing.
    /// Default implementation does nothing.
    fn reset(&mut self) {
        // Default: no-op
    }
}

// Blanket implementation for boxed trait objects
impl EntropySource for Box<dyn EntropySource> {
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        (**self).fill_bytes(dest)
    }
    
    fn name(&self) -> &str {
        (**self).name()
    }
}

// Blanket implementation for mutable references
impl<T: ?Sized + EntropySource> EntropySource for &mut T {
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        (**self).fill_bytes(dest)
    }

    fn name(&self) -> &str {
        (**self).name()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_system_entropy() {
        let mut entropy = SystemEntropy::new();
        let mut buf = [0u8; 32];
        entropy.fill_bytes(&mut buf);
        
        // Should not be all zeros (probability is negligible)
        assert!(buf.iter().any(|&b| b != 0));
    }
    
    #[test]
    fn test_next_methods() {
        let mut entropy = SystemEntropy::new();
        
        let byte = entropy.next_byte();
        let u32_val = entropy.next_u32();
        let u64_val = entropy.next_u64();
        
        // Just check they don't panic
        assert!(byte <= 255);
        assert!(u32_val <= u32::MAX);
        assert!(u64_val <= u64::MAX);
    }
}
