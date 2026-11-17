# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- The bundled application icon is used for the window, the in-app header and
  the README instead of a lock emoji
- Windows release builds embed `assets/icons/icon.ico` into the executable, so
  Explorer and the taskbar show the real icon
- Linux release archives include a `.desktop` entry and a 256x256 hicolor icon

### Changed
- Release builds on Windows no longer open a console window next to the GUI
- The window declares the `entropy-forge` app id, so Wayland panels can match
  it with the installed `.desktop` entry
- Upgraded `statrs` to 0.19, which drops the unmaintained `paste` transitive
  dependency (RUSTSEC-2024-0436)

### Removed
- Unused direct dependencies: `rand`, `thiserror` and `criterion`

## [0.2.0] - 2026-11-15

### Added
- Interactive "Learn" tab with step-by-step XOR visualization
- Bit-level encryption visualization
- Auto-play animation with speed control
- Educational explanations for cryptographic concepts
- `KeyedStreamCipher`: real ChaCha20 encryption/decryption with a 256-bit key
  and 96-bit nonce, plus SHA-256 passphrase key derivation
- `AvalancheAnalyzer`: real SHA-256 strict avalanche criterion analysis
- Shared zero-allocation MSB-first bit reader (`quality::bits`)
- NIST SP 800-22 Block Frequency and Cumulative Sums tests
- NIST SP 800-90B MCV min-entropy (99% confidence bound), Markov-1
  conditional min-entropy and lag-1 serial correlation metrics
- `tests/crypto_nist_golden.rs` NIST golden vectors and adversarial cases
- `tests/mobile_ui_test.rs` headless 360x800 narrow-screen rendering test
- Use tab: explicit Encrypt/Decrypt actions and a "Ciphertext as hex" option
  that describes only the ciphertext representation
- Shared GUI spacing tokens (`FIELD_GAP`, `SECTION_GAP`) for consistent form
  layout across screens
- Widget-level encrypt/decrypt round-trip test and 360 px bounding-box
  assertions for every tab

### Changed
- Rewrote the Runs, Longest Run, Serial and Frequency tests against the NIST
  SP 800-22 algorithms and validated them with the official sample vectors
- Quality score is capped at 30 for deterministic or structurally unsound
  sequences, even when Shannon entropy is 8.0
- `Test` and `Benchmark` jobs run on background threads instead of blocking the
  UI thread
- Learn tab now operates on UTF-8 bytes instead of truncating characters, and
  reports entropy against the absolute 8 bits/byte scale
- GUI migrated to egui/eframe 0.37 and egui_kittest 0.37
- Text fields and sliders expose accessible labels; fixed widths that overflow
  a 360 px phone screen were removed

### Fixed
- Decrypting in the GUI no longer hex-encodes the recovered plaintext; the hex
  option applies only to the ciphertext representation
- Invalid hex ciphertext is reported instead of being silently treated as raw
  text bytes

### Removed
- Misleading `avalanche_effect` method on the educational XOR `StreamCipher`
  (XOR has no plaintext avalanche effect by construction)

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

[Unreleased]: https://github.com/rvoidex7/entropy-forge/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/rvoidex7/entropy-forge/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/rvoidex7/entropy-forge/releases/tag/v0.1.0
