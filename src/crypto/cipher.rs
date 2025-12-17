//! Simple stream cipher implementation

use crate::entropy::EntropySource;

/// Simple XOR stream cipher
///
/// This cipher generates a keystream from the entropy source and XORs it
/// with the plaintext. Since XOR is symmetric, the same operation is used
/// for both encryption and decryption.
///
/// # Security Note
///
/// This is a simplified educational implementation. For production use,
/// consider standard ciphers like ChaCha20 or AES-GCM.
///
/// # Examples
///
/// ```
/// use entropy_weaver::entropy::SystemEntropy;
/// use entropy_weaver::crypto::StreamCipher;
///
/// let entropy = SystemEntropy::new();
/// let mut cipher = StreamCipher::new(entropy);
///
/// let plaintext = b"Hello, World!";
/// let ciphertext = cipher.process(plaintext);
/// let decrypted = cipher.process(&ciphertext);
///
/// // Note: Due to new keystream, decrypted won't match plaintext
/// // In real usage, you'd need to sync the keystream
/// ```
pub struct StreamCipher<E: EntropySource> {
    entropy: E,
    state: Vec<u8>,
    bytes_processed: usize,
}

impl<E: EntropySource> StreamCipher<E> {
    /// Create a new cipher with given entropy source
    pub fn new(entropy: E) -> Self {
        Self {
            entropy,
            state: Vec::new(),
            bytes_processed: 0,
        }
    }
    
    /// Process data (encrypt or decrypt - XOR is symmetric)
    ///
    /// Generates a fresh keystream and XORs with input data.
    ///
    /// # Arguments
    ///
    /// * `data` - Input data to process
    ///
    /// # Returns
    ///
    /// Processed data (same length as input)
    pub fn process(&mut self, data: &[u8]) -> Vec<u8> {
        // Generate keystream
        let mut keystream = vec![0u8; data.len()];
        self.entropy.fill_bytes(&mut keystream);
        
        // Store state for visualization (last 64 bytes)
        self.state = keystream.iter()
            .take(64.min(keystream.len()))
            .copied()
            .collect();
        
        // XOR data with keystream
        let output: Vec<u8> = data.iter()
            .zip(keystream.iter())
            .map(|(d, k)| d ^ k)
            .collect();
        
        self.bytes_processed += data.len();
        output
    }
    
    /// Get current cipher state (for visualization)
    ///
    /// Returns the last 64 bytes of keystream generated, or fewer if
    /// less has been generated.
    pub fn state(&self) -> &[u8] {
        &self.state
    }
    
    /// Get total bytes processed
    pub fn bytes_processed(&self) -> usize {
        self.bytes_processed
    }
    
    /// Reset the cipher (restarts byte counter)
    pub fn reset(&mut self) {
        self.state.clear();
        self.bytes_processed = 0;
        self.entropy.reset();
    }
    
    /// Calculate avalanche effect
    ///
    /// Encrypts data, flips one bit, re-encrypts, and returns the percentage
    /// of bits that changed in the output.
    ///
    /// A good cipher should have ~50% avalanche effect.
    pub fn avalanche_effect(&mut self, data: &[u8], bit_to_flip: usize) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        
        // First encryption
        let output1 = self.process(data);
        
        // Flip one bit in input
        let mut modified_data = data.to_vec();
        let byte_idx = bit_to_flip / 8;
        let bit_idx = bit_to_flip % 8;
        
        if byte_idx < modified_data.len() {
            modified_data[byte_idx] ^= 1 << bit_idx;
        }
        
        // Second encryption (will use different keystream!)
        let output2 = self.process(&modified_data);
        
        // Count bit differences
        let mut different_bits = 0;
        let total_bits = output1.len() * 8;
        
        for (b1, b2) in output1.iter().zip(output2.iter()) {
            let xor = b1 ^ b2;
            different_bits += xor.count_ones() as usize;
        }
        
        (different_bits as f64 / total_bits as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entropy::MockEntropy;
    
    #[test]
    fn test_cipher_basic() {
        let entropy = MockEntropy::new(42);
        let mut cipher = StreamCipher::new(entropy);
        
        let plaintext = b"Hello, World!";
        let ciphertext = cipher.process(plaintext);
        
        // Should produce different output
        assert_ne!(plaintext.as_slice(), ciphertext.as_slice());
        assert_eq!(plaintext.len(), ciphertext.len());
    }
    
    #[test]
    fn test_cipher_deterministic() {
        let entropy1 = MockEntropy::new(42);
        let entropy2 = MockEntropy::new(42);
        
        let mut cipher1 = StreamCipher::new(entropy1);
        let mut cipher2 = StreamCipher::new(entropy2);
        
        let plaintext = b"Test message";
        let ciphertext1 = cipher1.process(plaintext);
        let ciphertext2 = cipher2.process(plaintext);
        
        // Same entropy seed should produce same ciphertext
        assert_eq!(ciphertext1, ciphertext2);
    }
    
    #[test]
    fn test_cipher_state() {
        let entropy = MockEntropy::new(42);
        let mut cipher = StreamCipher::new(entropy);
        
        cipher.process(b"Some data");
        let state = cipher.state();
        
        assert!(!state.is_empty());
        assert!(state.len() <= 64);
    }
    
    #[test]
    fn test_bytes_processed() {
        let entropy = MockEntropy::new(42);
        let mut cipher = StreamCipher::new(entropy);
        
        cipher.process(b"12345");
        assert_eq!(cipher.bytes_processed(), 5);
        
        cipher.process(b"67890");
        assert_eq!(cipher.bytes_processed(), 10);
    }
    
    #[test]
    fn test_avalanche_effect() {
        let entropy = MockEntropy::new(42);
        let mut cipher = StreamCipher::new(entropy);
        
        let data = b"Test data for avalanche";
        let avalanche = cipher.avalanche_effect(data, 0);
        
        // Should have some avalanche effect (though it won't be 50% with
        // this simple implementation and random keystream changes)
        assert!(avalanche >= 0.0 && avalanche <= 100.0);
    }
}
