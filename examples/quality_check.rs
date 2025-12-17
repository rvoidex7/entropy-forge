//! Comprehensive quality check example

use entropy_weaver::entropy::{EntropySource, SystemEntropy};
use entropy_weaver::quality::{QualityMetrics, NistTests};

fn main() {
    println!("=== Entropy Quality Check ===\n");
    
    let mut entropy = SystemEntropy::new();
    println!("Testing: {}", entropy.name());
    println!("Sample size: 500,000 bytes\n");
    
    // Run quality metrics
    println!("--- Basic Metrics ---");
    let metrics = QualityMetrics::analyze(&mut entropy, 500_000);
    
    println!("Shannon Entropy:  {:.4} bits/byte (max: 8.0)", metrics.shannon_entropy);
    println!("Min-Entropy:      {:.4} bits/byte", metrics.min_entropy);
    println!("Mean byte value:  {:.2} (ideal: 127.5)", metrics.mean);
    println!("Chi-square stat:  {:.2}", metrics.chi_square);
    println!("Longest bit run:  {} bits", metrics.longest_run);
    println!("Overall Score:    {:.1}/100", metrics.overall_score());
    
    // Quality assessment
    println!("\nQuality Assessment:");
    if metrics.shannon_entropy >= 7.9 {
        println!("  ✓ Excellent entropy quality");
    } else if metrics.shannon_entropy >= 7.5 {
        println!("  ✓ Good entropy quality");
    } else {
        println!("  ⚠ Entropy quality could be improved");
    }
    
    // Run NIST tests
    println!("\n--- NIST SP 800-22 Tests ---");
    let mut data = vec![0u8; 500_000];
    entropy.fill_bytes(&mut data);
    let nist_results = NistTests::run_all_tests(&data);
    
    let mut passed = 0;
    for (name, p_value) in nist_results {
        let status = if p_value >= 0.01 {
            passed += 1;
            "✓ Pass"
        } else {
            "✗ Fail"
        };
        
        println!("{:<20} p={:.4}  {}", name, p_value, status);
    }
    
    println!("\nNIST Tests Passed: {}/5", passed);
    
    if passed >= 4 {
        println!("\n✓ Source passes statistical randomness tests");
    } else {
        println!("\n⚠ Source shows signs of non-randomness");
    }
    
    // Byte distribution
    println!("\n--- Byte Distribution Sample ---");
    println!("First 10 bytes: {}", 
        hex::encode(&data[..10]));
    println!("Middle 10 bytes: {}", 
        hex::encode(&data[250_000..250_010]));
    println!("Last 10 bytes: {}", 
        hex::encode(&data[data.len()-10..]));
    
    println!("\nQuality check complete!");
}
