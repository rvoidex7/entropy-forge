//! Basic usage example

use entropy_weaver::entropy::{EntropySource, SystemEntropy};
use entropy_weaver::crypto::StreamCipher;
use entropy_weaver::quality::QualityMetrics;

fn main() {
    println!("=== Entropy Weaver - Basic Usage ===\n");
    
    // Create entropy source
    let mut entropy = SystemEntropy::new();
    println!("Using entropy source: {}", entropy.name());
    
    // Generate some random bytes
    let mut buffer = [0u8; 16];
    entropy.fill_bytes(&mut buffer);
    println!("\nGenerated 16 random bytes:");
    println!("{}", hex::encode(buffer));
    
    // Use with stream cipher
    println!("\n--- Stream Cipher Example ---");
    let mut cipher = StreamCipher::new(SystemEntropy::new());
    let plaintext = b"Hello, Entropy Weaver!";
    let ciphertext = cipher.process(plaintext);
    println!("Plaintext:  {}", String::from_utf8_lossy(plaintext));
    println!("Ciphertext: {}", hex::encode(&ciphertext));
    
    // Analyze quality
    println!("\n--- Quality Analysis ---");
    let mut entropy_for_test = SystemEntropy::new();
    let metrics = QualityMetrics::analyze(&mut entropy_for_test, 10_000);
    
    println!("Shannon Entropy: {:.4} bits/byte", metrics.shannon_entropy);
    println!("Min-Entropy:     {:.4} bits/byte", metrics.min_entropy);
    println!("Mean byte value: {:.2}", metrics.mean);
    println!("Overall Score:   {:.1}/100", metrics.overall_score());
    
    println!("\nDone!");
}
