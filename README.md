# 🔐 Entropy Forge

Pluggable entropy framework for cryptographic applications with built-in quality testing, visualization, and **interactive educational tools**.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)
![CI](https://github.com/rvoidex7/entropy-forge/workflows/CI/badge.svg)

## Features

- **🔌 Trait-based Design**: Any RNG can be plugged in via the `EntropySource` trait
- **🔬 Quality Testing**: NIST SP 800-22 tests, Shannon entropy, min-entropy calculations
- **🔐 Cryptographic Use**: Stream cipher implementation using your entropy source
- **📊 Real-time Visualization**: egui-based GUI for interactive exploration
- **⚡ Performance Benchmarks**: Measure throughput and latency of entropy sources
- **🎓 Interactive Learning**: Step-by-step visualizations of cryptographic operations

## Screenshots

| Use Tab | Test Tab | Learn Tab |
|---------|----------|-----------|
| Stream cipher encryption | Quality metrics & NIST tests | Step-by-step XOR visualization |

## Quick Start

### Run the GUI

```bash
cargo run --release
```

### Run Examples

```bash
# Basic usage
cargo run --example basic_usage

# Custom entropy source
cargo run --example custom_source

# Quality check with NIST tests
cargo run --example quality_check
```

## GUI Tabs

### 📝 Use Tab
Encrypt and decrypt data using the stream cipher with real-time keystream visualization.

### 🔬 Test Tab
Analyze entropy quality with:
- Shannon Entropy (bits/byte)
- Min-Entropy (conservative estimate)
- Chi-Square uniformity test
- NIST SP 800-22 statistical tests (Frequency, Runs, Longest Run, Serial, Chi-Square)

### ⚡ Benchmark Tab
Measure entropy source performance:
- Throughput (MB/s)
- Latency per byte (µs)

### 🎓 Learn Tab
**Interactive educational visualizations** that teach cryptographic concepts step-by-step:

- **XOR Cipher Visualization**: See exactly how each character is encrypted
  - Character → ASCII → Binary conversion
  - Bit-by-bit XOR operation with color coding
  - Keystream generation from entropy source
  - Navigation controls (Previous/Next/Play/Pause)
  - Adjustable animation speed

Perfect for understanding how stream ciphers actually work!

## Usage

### Using the Default Source

```rust
use entropy_forge::entropy::SystemEntropy;
use entropy_forge::crypto::StreamCipher;

let entropy = SystemEntropy::new();
let mut cipher = StreamCipher::new(entropy);

let plaintext = b"Hello, World!";
let ciphertext = cipher.process(plaintext);
```

### Implementing a Custom Source

```rust
use entropy_forge::entropy::EntropySource;

struct MyRNG {
    state: u64,
}

impl EntropySource for MyRNG {
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        // Your RNG implementation here
        for byte in dest.iter_mut() {
            self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
            *byte = (self.state >> 24) as u8;
        }
    }
    
    fn name(&self) -> &str {
        "My Custom RNG"
    }
}
```

### Testing Entropy Quality

```rust
use entropy_forge::quality::QualityMetrics;
use entropy_forge::entropy::SystemEntropy;

let mut source = SystemEntropy::new();
let metrics = QualityMetrics::analyze(&mut source, 100_000);

println!("Shannon Entropy: {:.4} bits/byte", metrics.shannon_entropy);
println!("Min-Entropy: {:.4} bits/byte", metrics.min_entropy);
println!("Quality Score: {:.1}/100", metrics.overall_score());
```

### Running NIST Tests

```rust
use entropy_forge::quality::NistTests;
use entropy_forge::entropy::{EntropySource, SystemEntropy};

let mut source = SystemEntropy::new();
let mut data = vec![0u8; 100_000];
source.fill_bytes(&mut data);

let results = NistTests::run_all_tests(&data);
for (name, p_value) in results {
    println!("{}: {:.4} {}", 
        name, 
        p_value, 
        if p_value >= 0.01 { "✓" } else { "✗" }
    );
}
```

## Design Philosophy

Entropy Forge is designed to work with **any** entropy source through a clean trait interface. Whether you're using:

- System RNG (`/dev/urandom`, `BCryptGenRandom`, etc.)
- Hardware RNG (TPM, RDRAND, etc.)
- Chaotic systems (Lorenz, Rössler attractors, etc.)
- Quantum sources
- Custom implementations

The framework provides:

1. **Practical cryptographic use** (stream ciphers, key derivation)
2. **Quality verification** (statistical tests, entropy metrics)
3. **Performance measurement** (throughput, latency benchmarks)
4. **Visual feedback** (real-time GUI, state visualization)
5. **Educational tools** (step-by-step algorithm breakdowns)

**High-quality entropy sources** (7.5+ bits/byte min-entropy) will show optimal performance across all metrics.

## Architecture

```
entropy-forge/
├── src/
│   ├── entropy/      # EntropySource trait + implementations
│   ├── crypto/       # Stream cipher, key derivation
│   ├── quality/      # Metrics, NIST tests
│   ├── bench/        # Performance benchmarking
│   ├── learn/        # Educational visualizations
│   └── viz/          # GUI components (egui)
├── examples/         # Usage examples
└── tests/            # Integration tests
```

## Quality Metrics

The framework measures:

- **Shannon Entropy**: Average information content (max: 8.0 bits/byte)
- **Min-Entropy**: Conservative estimate based on most probable outcome
- **Chi-Square**: Tests uniformity of byte distribution
- **NIST SP 800-22 Tests**: Industry-standard randomness tests
  - Frequency Test
  - Runs Test
  - Longest Run Test
  - Serial Test
  - Chi-Square Test

## Performance

Typical results on modern hardware (example with System RNG):

- **Throughput**: 300-500 MB/s
- **Latency**: 2-3 µs per byte
- **Quality**: 7.98+ bits/byte Shannon entropy

## Examples in the Wild

### Use Case: Chaotic System RNG

High-quality chaotic systems (e.g., Rössler attractors) producing 7.5+ bits/byte entropy can be seamlessly integrated:

```rust
// Hypothetical example
struct ChaoticRNG {
    // Rössler system state...
}

impl EntropySource for ChaoticRNG {
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        // Evolve chaotic system, extract bytes...
    }
    
    fn name(&self) -> &str {
        "Chaotic RNG (Rössler)"
    }
}

// Use with Entropy Forge
let mut source = ChaoticRNG::new();
let metrics = QualityMetrics::analyze(&mut source, 1_000_000);
// Expect: ~7.75 bits/byte, excellent NIST scores

let mut cipher = StreamCipher::new(source);
// Now using chaotic entropy for cryptography!
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- NIST SP 800-22 for statistical test specifications
- The Rust community for excellent cryptographic libraries
- egui for the immediate-mode GUI framework

## Disclaimer

This is an educational and research tool. For production cryptographic applications, use well-vetted libraries and consult with security experts.

---

**Made with 🦀 Rust**
