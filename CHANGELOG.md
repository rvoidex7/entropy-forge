# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Interactive "Learn" tab with step-by-step XOR visualization
- Bit-level encryption visualization
- Auto-play animation with speed control
- Educational explanations for cryptographic concepts

## [0.1.0] - 2025-01-XX

### Added
- Initial release
- Pluggable `EntropySource` trait system
- System entropy implementation (Windows BCryptGenRandom, Linux /dev/urandom, macOS SecRandomCopyBytes)
- Stream cipher implementation with XOR keystream
- Quality metrics (Shannon entropy, min-entropy, chi-square)
- NIST SP 800-22 statistical tests (Frequency, Runs, Longest Run, Chi-Square, Serial)
- Performance benchmarking
- egui-based GUI with four tabs:
  - **Use**: Stream cipher encryption/decryption
  - **Test**: Quality analysis and NIST tests
  - **Benchmark**: Performance measurement
  - **Learn**: Educational visualization
- Cross-platform support (Windows, Linux, macOS)

### Security
- Note: This is an educational tool. Do not use for production cryptography.

---

[Unreleased]: https://github.com/rvoidex7/entropy-forge/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/rvoidex7/entropy-forge/releases/tag/v0.1.0
