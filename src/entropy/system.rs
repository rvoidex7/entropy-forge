//! System entropy source using OS random number generator

use super::EntropySource;
use getrandom::getrandom;

/// System entropy source (uses OS RNG)
///
/// This implementation uses the operating system's cryptographically secure
/// random number generator:
/// - Linux/Android: `/dev/urandom`
/// - macOS/iOS: `SecRandomCopyBytes`
/// - Windows: `BCryptGenRandom`
///
/// This is the default entropy source and provides good quality randomness
/// for most applications.
///
/// # Examples
///
/// ```
/// use entropy_weaver::entropy::{EntropySource, SystemEntropy};
///
/// let mut entropy = SystemEntropy::new();
/// let mut buffer = [0u8; 32];
/// entropy.fill_bytes(&mut buffer);
/// ```
#[derive(Debug, Default, Clone)]
pub struct SystemEntropy;

impl SystemEntropy {
    /// Create a new system entropy source
    pub fn new() -> Self {
        Self
    }
}

impl EntropySource for SystemEntropy {
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        getrandom(dest).expect("Failed to get system entropy - this should never happen on supported platforms");
    }
    
    fn name(&self) -> &str {
        #[cfg(target_os = "linux")]
        return "System RNG (Linux /dev/urandom)";
        
        #[cfg(target_os = "macos")]
        return "System RNG (macOS SecRandomCopyBytes)";
        
        #[cfg(target_os = "windows")]
        return "System RNG (Windows BCryptGenRandom)";
        
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        return "System RNG";
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_system_entropy_basic() {
        let mut entropy = SystemEntropy::new();
        let mut buf1 = [0u8; 32];
        let mut buf2 = [0u8; 32];
        
        entropy.fill_bytes(&mut buf1);
        entropy.fill_bytes(&mut buf2);
        
        // Should produce different output each time
        assert_ne!(buf1, buf2);
    }
    
    #[test]
    fn test_system_entropy_large() {
        let mut entropy = SystemEntropy::new();
        let mut buf = vec![0u8; 1_000_000];
        
        entropy.fill_bytes(&mut buf);
        
        // Check that we have good distribution (not all zeros)
        let zeros = buf.iter().filter(|&&b| b == 0).count();
        let expected_zeros = 1_000_000 / 256; // ~3906
        
        // Should be roughly 1/256 of bytes (allow 2x variance)
        assert!(zeros > expected_zeros / 2);
        assert!(zeros < expected_zeros * 2);
    }
    
    #[test]
    fn test_name() {
        let entropy = SystemEntropy::new();
        let name = entropy.name();
        assert!(name.contains("System RNG"));
    }
}
